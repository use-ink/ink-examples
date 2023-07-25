/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{js,ts,tsx}', '../../ui/**/*.{js,ts,tsx}'],
  theme: {
    extend: {
      fontFamily: {
        montserrat: ['Montserrat'],
        orbitron: ['Orbitron'],
      },
      colors: {
        brand: {
          300: '#E2DCFE',
          450: '#D9B7FF',
          500: '#BD83FD',
          550: '#AD63FF',
          600: '#7967EB',
          800: '#5a007e',
          900: '#4030a3',
          950: '#1A1452',
          1000: '#0C082B',
        },
        background: {
          100: '#F2F2F3',
          300: '#D6D8DC',
          700: '#444950',
          800: '#242526',
          850: '#1b1b1d',
          900: '#090909',
        },
        success: {
          500: '#00c900',
        },
        warning: {
          500: '#FFBE54',
          600: '#DE493E',
          700: '#B23A32',
        },
        error: {
          500: '#d6502b',
        },
        info: {
          500: '#bc83fb',
        },
      },
      maxWidth: {
        biggest: '1440px',
      },
      backgroundImage: {
        'brand-gradient':
          'linear-gradient(244.1deg, #9C45FC 12.55%, #A95DFC 32.31%, #BD83FD 86.37%)',
        'gradient-1': 'linear-gradient(180deg, #FFFFFF 0%, #EFEFEF 100%);',
        'gradient-1-dark': 'linear-gradient(180deg, #1B1B1D 0%, #242526 100%);',
      },
    },
    keyframes: {
      flicker: {
        '0%': 'opacity: random()',
        '50%': 'opacity: random()',
        '100%': 'opacity: random()',
      },
    },
  },
  plugins: [],
};
