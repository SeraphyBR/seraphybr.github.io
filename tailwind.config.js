/** @type {import('tailwindcss').Config} */
module.exports = {
  prefix: 'tw-',
  content: {
    files: ['./index.html', './src/**/*.rs'],
  },
  darkMode: 'selector',
  theme: {
    extend: {
      fontFamily: {
        sans: ['"Fira Sans"'],
        mono: ['"Fira Code"'],
      },
      typography: {
        DEFAULT: {
          css: {
            pre: false,
            code: false,
            'pre code': false,
            'code::before': false,
            'code::after': false,
          },
        },
      },
      colors: {
        graphite: '#222',
        accent: {
          DEFAULT: '#348314',
          light: '#56B045',
        },
      },
      backgroundImage: {
        'forest-stairs': "url('/img/background-1.jpg')",
        'road-trees': "url('/img/background-2.jpg')",
      },
      animation: {
        rotateIn: 'rotateIn 1s both',
      },
      keyframes: {
        rotateIn: {
          '0%': {
            transform: 'rotate3d(0,0,1,-200deg)',
            transformOrigin: 'center',
            opacity: '0',
          },
          '100%': {
            transform: 'none',
            transformOrigin: 'center',
            opacity: '1',
          },
        },
      },
    },
  },
  plugins: [require('@tailwindcss/typography'), require('./tailwind.utils')],
};
