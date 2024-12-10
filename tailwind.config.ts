import type { Config } from 'tailwindcss';

function generatePixelObject(max: number) {
  return Array.from(Array(max)).reduce((acc, _, i) => {
    acc[i] = `${i}px`;
    return acc;
  }, {}) as Record<string, string>;
}

const px0_10 = generatePixelObject(11);
const px0_100 = generatePixelObject(101);
const px0_200 = generatePixelObject(201);
const px0_2000 = generatePixelObject(2001);

export default {
  content: [
    './src/pages/**/*.{js,ts,jsx,tsx,mdx}',
    './src/components/**/*.{js,ts,jsx,tsx,mdx}',
    './src/app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      borderWidth: px0_10,
      fontSize: px0_100,
      lineHeight: px0_100,
      minWidth: px0_2000,
      minHeight: px0_2000,
      maxWidth: px0_2000,
      maxHeight: px0_2000,
      spacing: px0_200,
      borderRadius: px0_100,
      width: px0_2000,
      height: px0_2000,
      inset: px0_2000,
    },
  },
  plugins: [],
} satisfies Config;
