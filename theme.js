// ensure that our theme is set to the correct one
if (localStorage.getItem('mdbook-theme') !== 'discord') {
    localStorage.setItem('mdbook-theme', 'discord');
    location.reload();
}

// disable themes (they are lame)
let theme_button = document.getElementById('theme-toggle');
theme_button.remove();
