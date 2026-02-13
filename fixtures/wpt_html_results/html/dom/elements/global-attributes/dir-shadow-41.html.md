# html/dom/elements/global-attributes/dir-shadow-41.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/dir-shadow-41.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8"/>
<title>[dir] and shadow slots: dir=ltr on a div in the shadow tree, dir=rtl on the shadow host, no slotted text in the light or dark trees</title>
<link rel="author" title="Eric Meyer" href="mailto:emeyer@igalia.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/C#the-dir-attribute">
<link rel="help" href="https://github.com/whatwg/html/issues/3699">
<link rel="help" href="https://github.com/whatwg/html/pull/9796">
<link rel="match" href="dir-shadow-41-ref.html">
<style type="text/css">
body {width: 600px;}
#host {border: 1px solid gray; margin: 1em; padding: 0.25em;}
span {border: 1px solid silver;}

</style>
<script src="dir-shadow-utils.js"></script>
</head>
<body>

<p>`dir=rtl` on a div in the shadow tree, `dir=ltr` on the shadow host, no slotted text in the light or dark trees</p>
<div id="host" dir="ltr"><span slot="x1"></span></div>
<p id="result">The HTML direction / computed CSS `direction` value is: </p>

<script type="text/javascript">
  let root = host.attachShadow({mode:"open"});
  root.innerHTML = `<div dir="rtl"><slot dir="auto" name="x1"></slot></div>`;
  result.innerHTML += html_direction(root.querySelector("div[dir=rtl]").firstChild) + " / " + getComputedStyle(root.querySelector("div[dir=rtl]").firstChild).direction + " (on the " +  root.querySelector("div[dir=rtl]").firstChild.localName + ').';
</script>

</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 562,
        "byte_start": 539,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 1031,
        "byte_start": 1000,
        "col": 1,
        "line": 25
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/dom/elements/global-attributes/dir-shadow-41.html"
}
```
