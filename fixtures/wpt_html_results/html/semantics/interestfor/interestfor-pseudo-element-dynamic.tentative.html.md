# html/semantics/interestfor/interestfor-pseudo-element-dynamic.tentative.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestfor-pseudo-element-dynamic.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://open-ui.org/components/interest-invokers.explainer">
<link rel="match" href="interestfor-pseudo-element-dynamic-ref.html">
<p>You should see a button with the text PASS below.</p>
<button id="interest" interestfor="target">PASS</button>
<div id="target"></div>
<style>
  ::interest-hint { content: "FAIL"; }
</style>
<script>
  interest.offsetTop;
  interest.removeAttribute("interestfor");
</script>
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
        "byte_end": 315,
        "byte_start": 308,
        "col": 1,
        "line": 7
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
  "source_name": "html/semantics/interestfor/interestfor-pseudo-element-dynamic.tentative.html"
}
```
