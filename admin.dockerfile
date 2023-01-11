from nginx:alpine
copy dist /usr/share/nginx/html
copy default.conf /etc/nginx/conf.d/default.conf