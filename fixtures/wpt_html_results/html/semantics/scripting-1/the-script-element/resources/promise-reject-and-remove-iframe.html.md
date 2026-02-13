# html/semantics/scripting-1/the-script-element/resources/promise-reject-and-remove-iframe.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/resources/promise-reject-and-remove-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script>
const promise = Promise.reject();

window.onload = () => {
  promise.catch(() => parent.document.querySelector('iframe').remove());
};
</script>

<!-- Load a slow script to delay window.onload for a while.
     Without this, crashes are flaky. -->
<script src="/common/slow.py"></script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/resources/promise-reject-and-remove-iframe.html"
}
```
