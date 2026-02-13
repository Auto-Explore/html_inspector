# html/editing/the-hidden-attribute/hidden-until-found-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/the-hidden-attribute/hidden-until-found-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>

<html class="reftest-wait">
<meta charset="utf8">
<title>content-visibility changes after a delay</title>
<link rel="author" title="Vladimir Levin" href="mailto:vmpstr@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#attr-hidden-until-found">
<link rel="match" href="./resources/container-ref.html">
<meta name="assert" content="scrollIntoView has no effect on hidden=until-found">
<script src="/common/reftest-wait.js"></script>

<style>
#container {
  width: 150px;
  height: 150px;
  background: lightblue;
}
#child {
  width: 50px;
  height: 50px;
  background: lightgreen;
}
</style>

<div id=container hidden=until-found>
  Text.
  <div id=child></div>
</div>


<script>

function runTest() {
  document.getElementById("container").scrollIntoView();
  takeScreenshot();
}

window.onload = requestAnimationFrame(() => {
  requestAnimationFrame(() => {
    requestAnimationFrame(() => {
      runTest();
    });
  });
});

</script>
</html>


```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/editing/the-hidden-attribute/hidden-until-found-001.html"
}
```
