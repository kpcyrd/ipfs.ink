# ipfs.ink [![travis][travis-image]][travis-url]

[travis-image]: https://img.shields.io/travis/kpcyrd/ipfs.ink/master.svg
[travis-url]: https://travis-ci.org/kpcyrd/ipfs.ink

Publish and render markdown essays to and from ipfs.

## Example

Assuming you wrote this essay:

    # demo

    this is my **example** text

    this is a [link](/ohai).

    ```
    this is some **code**.
    ```

And you want to share it with the world. With ipfs.ink, you can just write your essay, review the preview and then publish with a single click. Your essay is added to ipfs and you receive a unique link that you can send to other people.

They are then able to read a nicely rendered version of your markdown essay.

For example, the short text above can be viewed here:

```
https://ipfs.ink/e/QmULtKYfv5CSGeesjDiBQbfoB3S1F7QY33Dpu59HzRSATu
```

If you found this project, you might already know about ipfs. If you don't know why you would want to publish to ipfs, read it on [ipfs.io](https://ipfs.io/).

## Development

Run `./index.js` in one terminal, run `WEBPACK_DEV=1 cargo run` in another.

## Status

This project is in an early phase. There are some workarounds in the code and there is no guarantee of availability whatsoever. Please use this service for testing only. Everything you publish is publicly accessible.

## License

GPLv3
