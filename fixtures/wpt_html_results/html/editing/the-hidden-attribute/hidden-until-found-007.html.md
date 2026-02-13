# html/editing/the-hidden-attribute/hidden-until-found-007.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/the-hidden-attribute/hidden-until-found-007.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>

<html class="reftest-wait">
<meta charset="utf8">
<title>hidden=until-found + focus</title>
<link rel="author" title="Vladimir Levin" href="mailto:vmpstr@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#attr-hidden-until-found">
<link rel="match" href="./resources/spacer-and-container-ref.html">
<meta name="assert" content="focus does not scroll or focus element under hidden=until-found">
<script src="/common/reftest-wait.js"></script>

<style>
.spacer {
  width: 150px;
  height: 3000px;
  background: lightblue;
}
#container {
  width: 150px;
  height: 150px;
  background: lightblue;
}
#target {
  position: relative;
  top: 75px;

  width: 50px;
  height: 50px;
  background: red;
}
</style>

<div class=spacer></div>
<div id=container hidden=until-found>
  <div id=target tabindex=0></div>
</div>

<script>
function runTest() {
  document.getElementById("target").focus();
  requestAnimationFrame(takeScreenshot);
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
  "source_name": "html/editing/the-hidden-attribute/hidden-until-found-007.html"
}
```
