# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/sticky-content-crash.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/sticky-content-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="http://crbug.com/1146872">
<body>
<fieldset><span><span></span></span></fieldset>
<div id="host"><span></span></div>
<script>
const host = document.querySelector('#host');
const shadowRoot = host.attachShadow({mode: 'closed'});
const fieldset = shadowRoot.appendChild(document.createElement('fieldset'));
fieldset.setAttribute('style', 'overflow: scroll');
fieldset.innerHTML = '<slot></slot>';
</script>
<style>
*:not(fieldset, div) {
  position: sticky;
  bottom: 72pc;
}
fieldset {
  overflow: visible scroll;
}
</style>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 451,
        "byte_start": 444,
        "col": 1,
        "line": 13
      }
    },
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/sticky-content-crash.html"
}
```
