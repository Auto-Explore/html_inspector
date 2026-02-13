# html/rendering/replaced-elements/embedded-content/change-src-while-not-displayed.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/embedded-content/change-src-while-not-displayed.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<link rel="author" title="Morten Stenshorne" href="mailto:mstensho@chromium.org">
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=1240261">
<link rel="match" href="change-src-while-not-displayed-ref.html">
<embed id="embed" style="display:block;" src="data:text/html,FAIL">
<script>
  onload = function() {
    document.body.offsetTop;
    embed.style.display = "none";
    document.body.offsetTop;
    embed.src = "data:text/html,PASS";
    document.body.offsetTop;
    embed.style.display = "block";
    requestAnimationFrame(()=> {
      requestAnimationFrame(()=> {
        document.documentElement.classList.remove('reftest-wait');
      });
    });
  }
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
  "source_name": "html/rendering/replaced-elements/embedded-content/change-src-while-not-displayed.html"
}
```
