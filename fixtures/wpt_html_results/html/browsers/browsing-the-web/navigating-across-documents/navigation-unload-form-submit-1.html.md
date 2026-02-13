# html/browsers/browsing-the-web/navigating-across-documents/navigation-unload-form-submit-1.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/navigation-unload-form-submit-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>

<h1>navigation-unload-form-submit-1.html</h1>

<script>
let isSubmit = false;

window.unload = function () {
    window.location = 'is-Submit' + isSubmit;
}

function setIsSubmit() {
    isSubmit = true;
}
</script>

<form onsubmit="setIsSubmit" action="navigation-unload-form-submit-2.html">
  <input type="submit">
</form>

<script>
parent.finishedLoading();
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/navigation-unload-form-submit-1.html"
}
```
