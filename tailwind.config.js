/** @type {import('tailwindcss').Config} */
module.exports = {
  prefix: 'tw-',
  content: {
    files: ['./index.html', './src/**/*.rs'],
  },
  theme: {
    extend: {
      fontFamily: {
        sans: ['"Fira Sans"'],
      },
      colors: {
        graphite: '#222',
        greenLime: '#348314',
      },
      backgroundImage: {
        'forest-stairs': "url('/img/background-1.jpg')",
      },
      animation: {
        rotateIn: 'rotateIn 1s both'
      },
      keyframes: {
        rotateIn: {
          '0%': {
            transform: 'rotate3d(0,0,1,-200deg)',
            transformOrigin: 'center',
            opacity: '0'
          },
          '100%': {
            transform: 'none',
            transformOrigin: 'center',
            opacity: '1'
          }
        }
      }
    },
  },
  plugins: [require('@tailwindcss/typography')],
};
