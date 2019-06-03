export function timeago(date: Date): string {
  let val = (date || new Date()).getTime()
  val = 0 | ((Date.now() - val) / 1000)
  val = val < 0 ? 0 : val

  let unit, result
  let length: { [key: string]: number } = {
    second: 60,
    minute: 60,
    hour: 24,
    day: 7,
    week: 4.35,
    month: 12,
    year: 10000,
  }

  for (unit in length) {
    result = val % length[unit]
    if (!(val = 0 | (val / length[unit])))
      return result + " " + (result - 1 ? unit + "s" : unit)
  }

  return "invalid"
}
