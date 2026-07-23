const CHINA_TIME_ZONE = "Asia/Shanghai";
const WEEKDAYS = new Set(["Mon", "Tue", "Wed", "Thu", "Fri"]);

const chinaClockFormatter = new Intl.DateTimeFormat("en-US", {
  timeZone: CHINA_TIME_ZONE,
  weekday: "short",
  hour: "2-digit",
  minute: "2-digit",
  second: "2-digit",
  hourCycle: "h23",
});

interface MarketClock {
  weekday: string;
  seconds: number;
}

function getChinaMarketClock(now: Date): MarketClock {
  const values = new Map(
    chinaClockFormatter
      .formatToParts(now)
      .filter((part) => part.type !== "literal")
      .map((part) => [part.type, part.value])
  );
  const hour = Number(values.get("hour") ?? 0);
  const minute = Number(values.get("minute") ?? 0);
  const second = Number(values.get("second") ?? 0);
  return {
    weekday: values.get("weekday") ?? "",
    seconds: hour * 3600 + minute * 60 + second,
  };
}

function isWithinSession(
  seconds: number,
  startHour: number,
  startMinute: number,
  endHour: number,
  endMinute: number
): boolean {
  const start = startHour * 3600 + startMinute * 60;
  const end = endHour * 3600 + endMinute * 60;
  return seconds >= start && seconds <= end;
}

function isOpenOnWeekday(
  now: Date,
  morningEnd: [number, number],
  afternoonEnd: [number, number]
): boolean {
  const clock = getChinaMarketClock(now);
  if (!WEEKDAYS.has(clock.weekday)) return false;
  return (
    isWithinSession(clock.seconds, 9, 30, ...morningEnd) ||
    isWithinSession(clock.seconds, 13, 0, ...afternoonEnd)
  );
}

export function isFundTradingTime(now = new Date()): boolean {
  return isOpenOnWeekday(now, [11, 30], [15, 0]);
}

export function isStockTradingTime(code: string, now = new Date()): boolean {
  return code.toLowerCase().startsWith("hk")
    ? isOpenOnWeekday(now, [12, 0], [16, 0])
    : isOpenOnWeekday(now, [11, 30], [15, 0]);
}
