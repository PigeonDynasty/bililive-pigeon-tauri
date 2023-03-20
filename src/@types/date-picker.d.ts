import type { Dayjs } from 'dayjs'
import type { Writable } from 'svelte/store'
namespace DatePicker {
  interface Props {
    view: DatePickerSelectType
    date?: Dayjs
    showDate: Dayjs
  }
  interface Context {
    props: Writable<Props>
    time: Boolean
    select: Function
  }
}
