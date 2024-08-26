const plugin = require('tailwindcss/plugin');

module.exports = plugin(({ addUtilities, addComponents }) => {
  addUtilities({
    '.vflex': {
      '@apply tw-flex tw-flex-col': {},
    },
    '.hflex': {
      '@apply tw-flex tw-flex-row': {},
    },
    '.text-clickable-colors': {
      '@apply tw-text-gray-700 dark:tw-text-white hover:tw-text-accent dark:hover:tw-text-accent-light tw-transition-colors tw-duration-200':
        {},
    },
    '.border-clickable-colors': {
      '@apply tw-border-gray-200 dark:tw-border-gray-400 tw-transition tw-duration-200 hover:tw-border-accent dark:hover:tw-border-accent-light':
        {},
    },
  });

  addComponents({
    '.btn-primary': {
      '@apply tw-text-center tw-text-sm tw-font-bold tw-text-clickable-colors tw-border-solid tw-border-2 tw-rounded-xl tw-border-clickable-colors tw-max-w-fit tw-px-4 tw-py-2 tw-bg-white dark:tw-bg-graphite':
        {},
    },
  });
});
