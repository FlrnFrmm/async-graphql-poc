# NFT Backend
The NFT Marketplace Backend Application

Live Demo: https://iota-nft.herokuapp.com/ 

## Development

To run local:
```bash
cargo +nightly run
```
Visit http://localhost:8000/ in your browser.


## Documentation

To see the docs page, please run this command:
```bash
mdbook serve     
```
If you don't have mdbook installed, please check [this](./docs/README.md).


## Deployment

```bash
heroku login
heroku git:remote -a iota-nft
git push heroku main:master      
```

