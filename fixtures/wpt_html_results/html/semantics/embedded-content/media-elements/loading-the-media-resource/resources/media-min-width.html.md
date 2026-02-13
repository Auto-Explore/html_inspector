# html/semantics/embedded-content/media-elements/loading-the-media-resource/resources/media-min-width.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resources/media-min-width.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<video width="200"></video>
<script>
function createSource(src, media) {
  var source = document.createElement('source');
  source.src = src;
  if (media) {
    source.media = media;
  }
  return source;
}
const rAF = () => new Promise(resolve => requestAnimationFrame(resolve));
const hash = str => str.substr(str.lastIndexOf('#'));
(async () => {
  const v = document.querySelector('video');
  v.getBoundingClientRect(); // force layout flush. ensure viewport dimensions are up-to-date
  v.append(createSource('/media-source/mp4/test.mp4#a', '(min-width: 200px)'));
  v.append(createSource('/media-source/mp4/test.mp4#b'));
  await rAF();
  await rAF();
  window.resolveBeforeEnvChange(hash(v.currentSrc));
  window.frameElement.width = '150';
  await rAF();
  await rAF();
  window.resolveAfterEnvChange(hash(v.currentSrc));
  v.load()
  await rAF();
  await rAF();
  window.resolveAfterLoadCalled(hash(v.currentSrc));
})();
</script>
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resources/media-min-width.html"
}
```
