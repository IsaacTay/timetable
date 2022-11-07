import { publish } from 'gh-pages';

publish(
    'pkg',
    {
        dotfiles: true
    }
)