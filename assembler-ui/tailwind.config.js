/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'retro-green': '#00FF66',
        'cyber-yellow': '#FFF500',
        'vibrant-purple': '#FF007A',
        'electric-blue': '#00E0FF',
        'dark-bg': '#0A0A0A',
        'dark-bg-secondary': '#121212',
      },
      boxShadow: {
        '3d': '4px 4px 0px #000000',
        '3d-hover': '8px 8px 0px #000000',
        '3d-active': '0px 0px 0px #000000',
      },
      animation: {
        'scanline': 'scanline 0.15s steps(1)',
        'flash': 'flash 0.15s steps(1)',
      },
      keyframes: {
        scanline: {
          '0%': { transform: 'translateY(-100%)' },
          '100%': { transform: 'translateY(100%)' },
        },
        flash: {
          '0%, 100%': { filter: 'invert(0)' },
          '50%': { filter: 'invert(1)' },
        },
      },
    },
  },
  plugins: [],
}
