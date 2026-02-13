# html/semantics/interactive-elements/the-details-element/modified-details-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-details-element/modified-details-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<link rel=author href="mailto:vmpstr@chromium.org">
<link rel=help href="https://crbug.com/1276488">

<style>
.first + .second {}
</style>

<script>
const details = document.createElement('details');
const ol = document.createElement('ol');
const text = document.createTextNode('abcdefghijklmnopqrstuvxyz');

function step3() {
  ol.setAttribute('class', 'first');
}

function step2() {
  details.appendChild(text);
  requestAnimationFrame(step3);
}

function runTest() {
  document.documentElement.appendChild(details);
  details.appendChild(ol);

  requestAnimationFrame(step2);
}

onload = runTest;
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
  "source_name": "html/semantics/interactive-elements/the-details-element/modified-details-crash.html"
}
```
