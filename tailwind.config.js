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
        graphite: '#222',
        greenLime: '#348314'
      },
      backgroundImage: {
        'forest-stairs': "url('/img/background-1.jpg')"
      }
    },
  },
  plugins: [
    require('@tailwindcss/typography')
  ],
};
