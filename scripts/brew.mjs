import fs from 'fs-extra'
import crypto from 'crypto'
import progressStream from 'progress-stream'
import fetch from 'node-fetch'
import { getOctokit, context } from '@actions/github'
const BREW_NAME = 'BililivePigeon'
const FILE_FOLDER = '.files'
const RUBY_FILE = 'bililive-pigeon-tauri.rb'
const rubyData = {
  desc: 'bililive pigeon tauri.',
  homepage: 'https://github.com/PigeonDynasty/bililive-pigeon-tauri',
  platforms: {
    intel: {
      url: '',
      sha: ''
    },
    arm: {
      url: '',
      sha: ''
    }
  }
}
// 生成 homebrew ruby 文件
async function resolveBrew() {
  if (process.env.GITHUB_TOKEN === undefined) {
    throw new Error('GITHUB_TOKEN is required')
  }

  const options = { owner: context.repo.owner, repo: context.repo.repo }
  const github = getOctokit(process.env.GITHUB_TOKEN)

  const { data: tags } = await github.rest.repos.listTags({
    ...options,
    per_page: 10,
    page: 1
  })

  const tag = tags.find(t => t.name.startsWith('v'))

  const { data: latestRelease } = await github.rest.repos.getReleaseByTag({
    ...options,
    tag: tag.name
  })

  await exitsFolder(FILE_FOLDER)
  const promises = latestRelease.assets.map(async asset => {
    const { name, browser_download_url } = asset
    // darwin-x86_64 url (macos intel)
    if (name.endsWith('.app.tar.gz') && !name.includes('aarch')) {
      rubyData.platforms['intel'].url = browser_download_url
      rubyData.platforms['intel'].sha = await getSha256(browser_download_url)
    }

    // darwin-aarch64 url (macos silicon)
    if (name.endsWith('aarch64.app.tar.gz')) {
      rubyData.platforms['arm'].url = browser_download_url
      rubyData.platforms['arm'].sha = await getSha256(browser_download_url)
    }
  })

  await Promise.allSettled(promises)
  // const browser_download_url = 'https://github.com/PigeonDynasty/bililive-pigeon-tauri/releases/download/v0.0.3/Bililive.Pigeon_aarch64.app.tar.gz'
  // rubyData.platforms['arm'].url = browser_download_url
  // rubyData.platforms['arm'].sha = await getSha256(browser_download_url)
  // const download_url = 'https://github.com/PigeonDynasty/bililive-pigeon-tauri/releases/download/v0.0.3/Bililive.Pigeon_x64.app.tar.gz'
  // rubyData.platforms['intel'].url = download_url
  // rubyData.platforms['intel'].sha = await getSha256(download_url)
  const rubyStr = `class ${BREW_CLASS} < Formula
  name "${BREW_NAME}"
  desc "${rubyData.desc}"
  homepage "${rubyData.homepage}"
  version "${tag.name}"

  if Hardware::CPU.intel?
    url "${rubyData.platforms['inter'].url}"
    sha256 "${rubyData.platforms['inter'].sha}"
  else
    url "${rubyData.platforms['arm'].url}"
    sha256 "${rubyData.platforms['arm'].sha}"
  end
  
  def install
    prefix.install Dir["*"]
  end
  app "AltTab.app"
  def caveats
  <<~EOS
    You may need to enter the following code in the terminal to get it to work
    'sudo xattr -d com.apple.quarantine /Applications/Bililive\ Pigeon.app'
    EOS
  end
end`
  // 创建文件
  await exitsFolder('Formula')
  await fs.promises.writeFile(`Formula/${RUBY_FILE}`, rubyStr)
  console.log('create formula successfully')
}
async function exitsFolder(path) {
  try {
    await fs.promises.stat(path)
  } catch (e) {
    await fs.promises.mkdir(path)
  }
}
async function getSha256(url) {
  return new Promise(async (resolve, reject) => {
    const fileName = url.split('/').pop()
    const fileStream = fs.createWriteStream(`${FILE_FOLDER}/${fileName}`).on('error', (e) => {
      console.error('错误', e)
      reject(e)
    }).on('ready', () => {
      console.log("开始下载:", url)
    }).on('finish', async () => {
      console.log(fileName, '文件下载完成')
      resolve(createFileHash256(`${FILE_FOLDER}/${fileName}`))
    })

    console.log('请求文件：', url)
    const response = await fetch(url, {
      method: 'GET',
      headers: { 'Content-Type': 'application/octet-stream' }
    })
    // 获取请求头中的文件大小数据
    let length = response.headers.get("content-length");
    // 进度
    let str = progressStream({
      length, time: 100
    })
    str.on('progress', (progressData) => {
      let per = Math.round(progressData.percentage) + '%'
      console.log(fileName, per)
    })
    response.body.pipe(str).pipe(fileStream)

  })
}


function createFileHash256(file) {
  //读取一个Buffer
  const buffer = fs.readFileSync(file)
  const fsHash = crypto.createHash('sha256')

  fsHash.update(buffer)
  const md5 = fsHash.digest('hex')
  console.log(file, md5)
  return md5
}

resolveBrew().catch(console.error)
