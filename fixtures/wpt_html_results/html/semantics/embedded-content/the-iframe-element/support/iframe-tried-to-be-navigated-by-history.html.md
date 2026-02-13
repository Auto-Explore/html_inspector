# html/semantics/embedded-content/the-iframe-element/support/iframe-tried-to-be-navigated-by-history.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/support/iframe-tried-to-be-navigated-by-history.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<p>This is a frame that tries to navigate via history API.</p>
<script>
window.onmessage = (e) => {
  if (e.data == 'back') {
    history.back();
  } else if (e.data == 'forward') {
    history.forward();
  } else if (e.data = 'pushstateback') {
    onpopstate = (e) => {
      parent.postMessage('pushstatebackdone', '*');
    };

    history.pushState({someState: 'blah'}, '');
    history.back();
  }
};
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/support/iframe-tried-to-be-navigated-by-history.html"
}
```
