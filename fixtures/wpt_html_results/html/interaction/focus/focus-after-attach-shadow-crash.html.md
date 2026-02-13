# html/interaction/focus/focus-after-attach-shadow-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focus-after-attach-shadow-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://github.com/servo/servo/issues/42215">
<style>
  #outer { overflow: hidden; }
</style>
<div id="outer"><span id="inner" tabindex="1">hello</span></div>
<script>
  window.addEventListener("load", _ => {
    document.body.attachShadow({mode: "open"});
    inner.focus();
  });
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
  "source_name": "html/interaction/focus/focus-after-attach-shadow-crash.html"
}
```
