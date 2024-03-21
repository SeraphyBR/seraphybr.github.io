/** @type {import('tailwindcss').Config} */
module.exports = {
  prefix: 'tw-',
  content: {
    files: ['*.html', './src/**/*.rs'],
  },
  theme: {
    extend: {
      fontFamily: {
        sans: ['"Fira Sans"'],
      },
      colors: {
        graphite: '#222'
      }
    },
  },
  plugins: [],
};
