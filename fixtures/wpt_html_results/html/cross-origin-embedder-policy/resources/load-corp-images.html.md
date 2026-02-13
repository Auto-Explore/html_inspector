# html/cross-origin-embedder-policy/resources/load-corp-images.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-embedder-policy/resources/load-corp-images.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<script src="/common/get-host-info.sub.js"></script>
<script>

function remote(path) {
  const REMOTE_ORIGIN = get_host_info().HTTPS_REMOTE_ORIGIN;
  return new URL(path, REMOTE_ORIGIN);
}

let params = new URLSearchParams(location.search);
let token = params.get('token');
let revalidate = params.get('revalidate');

let image_path = `/html/cross-origin-embedder-policy/resources/corp-image.py?token=${token}&revalidate=${revalidate}`;

window.addEventListener("load", async () => {
  await new Promise(resolve => {
    let img = document.createElement("img");
    img.src = remote(image_path);
    img.onload = () => { window.parent.postMessage({corp: false, loaded: true}, "*"); resolve(); };
    img.onerror = (e) => { window.parent.postMessage({corp: false, loaded: false}, "*"); resolve(); };
    document.body.appendChild(img);
  });

  await new Promise(resolve => {
    let img = document.createElement("img");
    img.src = remote(image_path + "&corp-cross-origin=1");
    img.onload = () => { window.parent.postMessage({corp: true, loaded: true}, "*"); resolve(); };
    img.onerror = (e) => { window.parent.postMessage({corp: true, loaded: false}, "*"); resolve(); };
    document.body.appendChild(img);
  });

  window.parent.postMessage({done: true}, "*")
});

</script>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/cross-origin-embedder-policy/resources/load-corp-images.html"
}
```
