/** @type {import('tailwindcss').Config} */

const { addDynamicIconSelectors } = require('@iconify/tailwind')
const plugin = require('tailwindcss/plugin')

export default {
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    asideScrollbars: {
      light: 'light',
      gray: 'gray',
    },
    extend: {
      zIndex: {
        '-1': '-1',
      },
      flexGrow: {
        5: '5',
      },
      maxHeight: {
        'screen-menu': 'calc(100vh - 3.5rem)',
        modal: 'calc(100vh - 160px)',
      },
      transitionProperty: {
        position: 'right, left, top, bottom, margin, padding',
        textColor: 'color',
      },
      keyframes: {
        'fade-out': {
          from: { opacity: 1 },
          to: { opacity: 0 },
        },
        'fade-in': {
          from: { opacity: 0 },
          to: { opacity: 1 },
        },
        border: {
          to: { '--border-angle': '360deg' },
        },
      },
      animation: {
        'fade-out': 'fade-out 250ms ease-in-out',
        'fade-in': 'fade-in 250ms ease-in-out',
        border: 'border 4s linear infinite',
      },
    },
  },
  plugins: [
    addDynamicIconSelectors({
      // Prefix for selectors, must be different for each addDynamicIconSelectors()
      prefix: 'icon',
      // Removes redundant rules
      overrideOnly: false,
      // Icon height, 0 to disable size
      scale: 1,
      // Custom icon sets
      iconSets: {},
      // Callback to customize icons (such as change stroke-width, color, etc...).
      // First param is content, second is icon name, third is icon set prefix.
      // Function should return modified content.
      customise: (content, name, prefix) => content,
    }),
    plugin(function ({ matchUtilities, theme }) {
      matchUtilities(
        {
          'aside-scrollbars': (value) => {
            const track = value === 'light' ? '100' : '900'
            const thumb = value === 'light' ? '300' : '600'
            const color = value === 'light' ? 'gray' : value

            return {
              scrollbarWidth: 'thin',
              scrollbarColor: `${theme(`colors.${color}.${thumb}`)} ${theme(
                `colors.${color}.${track}`,
              )}`,
              '&::-webkit-scrollbar': {
                width: '8px',
                height: '8px',
              },
              '&::-webkit-scrollbar-track': {
                backgroundColor: theme(`colors.${color}.${track}`),
              },
              '&::-webkit-scrollbar-thumb': {
                borderRadius: '0.25rem',
                backgroundColor: theme(`colors.${color}.${thumb}`),
              },
            }
          },
        },
        { values: theme('asideScrollbars') },
      )
    }),
  ],
}
