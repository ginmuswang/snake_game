/**
 *
 * @param max 随机值的最大值
 * @returns 随机出来的数字
 */
export const random = (max: number): number => {
  return Math.floor(Math.random() * max);
};
