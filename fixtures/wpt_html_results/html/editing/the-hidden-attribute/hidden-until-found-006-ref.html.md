# html/editing/the-hidden-attribute/hidden-until-found-006-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/the-hidden-attribute/hidden-until-found-006-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<html>
<meta charset="utf8">
<title>content-visibility hidden-matchable + scrollIntoView (reference)</title>
<link rel="author" title="Vladimir Levin" href="mailto:vmpstr@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#attr-hidden-until-found">

<style>
.spacer {
  height: 1000px;
  background: lightgreen;
}
#container {
  width: 150px;
  height: 150px;
  background: lightblue;
}
#target {
  position: relative;
  top: 75px;
  visibility: hidden;

  width: 50px;
  height: 50px;
  background: red;
}
</style>

<div>top of the page</div>
<div class=spacer></div>
<div id=container>
  <div id=target></div>
</div>
<div class=spacer></div>
<div>bottom of the page</div>

<script>
onload = () => document.getElementById("target").scrollIntoView();
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
  "source_name": "html/editing/the-hidden-attribute/hidden-until-found-006-ref.html"
}
```
