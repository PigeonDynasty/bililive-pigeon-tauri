import fs from 'fs-extra'
import crypto from 'crypto'
import progressStream from 'progress-stream'
import fetch from 'node-fetch'
import { getOctokit, context } from '@actions/github'
const CASKS_FOLDER = "Casks"
const FILE_FOLDER = '.files'
const rubyData = {
  name: 'Bililive Pigeon',
  desc: 'bililive pigeon.',
  homepage: 'https://github.com/PigeonDynasty/bililive-pigeon-tauri',
  url: 'https://github.com/PigeonDynasty/bililive-pigeon-tauri/releases/download/v#{version}/Bililive.Pigeon_#{version}_#{arch}.dmg',
  sha: {
    intel: '',
    arm: ''
  }
}
const _name = rubyData.name.toLowerCase().replaceAll(' ', '-')
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
    if (name.endsWith('.dmg') && !name.includes('aarch')) {
      rubyData.sha['intel'] = await getSha256(browser_download_url)
    }

    // darwin-aarch64 url (macos silicon)
    if (name.endsWith('aarch64.dmg')) {
      rubyData.sha['arm'] = await getSha256(browser_download_url)
    }
  })

  await Promise.allSettled(promises)

  const rubyStr = `cask "${_name}" do
  arch arm: "aarch64", intel: "x64"
  version "${tag.name.substring(1)}"
  sha256  intel: ""${rubyData.sha['intel']}"",
          arm:   "${rubyData.sha['arm']}"
          
  url "${rubyData.url}"
  
  livecheck do
    url :url
  end

  auto_updates true

  name "${rubyData.name}"
  desc "${rubyData.desc}"
  homepage "${rubyData.homepage}"

  depends_on macos: ">= :catalina"

  app "${rubyData.name}.app"
  
  caveats <<~EOS
    You may need to enter the following code in the terminal to get it to work
    'sudo xattr -r -d com.apple.quarantine /Applications/${rubyData.name}.app'
  EOS
end
`
  // 创建文件
  await exitsFolder(CASKS_FOLDER)
  await fs.promises.writeFile(`${CASKS_FOLDER}/${_name}.rb`, rubyStr)
  console.log('create cask successfully')
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
