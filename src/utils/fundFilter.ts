export type FundCodeItem = { code: string };

export function isFundInList(funds: FundCodeItem[], code: string): boolean {
  if (!code) return false;
  return funds.some((fund) => fund.code === code);
}

export function filterFundsByCode<T extends FundCodeItem>(
  funds: T[],
  code: string | null
): T[] {
  if (!code) return funds;
  return funds.filter((fund) => fund.code === code);
}
