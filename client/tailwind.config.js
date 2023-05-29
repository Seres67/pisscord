/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["src/**/*.rs"],
    theme: {
        extend: {
            colors: {
                'background': {
                    '50': '#f7f7f7',
                    '100': '#e3e3e3',
                    '200': '#c8c8c8',
                    '300': '#a4a4a4',
                    '400': '#818181',
                    '500': '#666666',
                    '600': '#515151',
                    '700': '#434343',
                    '800': '#383838',
                    '900': '#313131',
                    '950': '#242424',
                },
                'text': {
                    '50': '#ffffff',
                    '100': '#efefef',
                    '200': '#dcdcdc',
                    '300': '#bdbdbd',
                    '400': '#989898',
                    '500': '#7c7c7c',
                    '600': '#656565',
                    '700': '#525252',
                    '800': '#464646',
                    '900': '#3d3d3d',
                    '950': '#292929',
                },

            }
        },
    },
    plugins: [],
}

