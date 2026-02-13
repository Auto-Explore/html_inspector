# html/rendering/replaced-elements/embedded-content/tall-cross-domain-iframe-in-scrolled.sub-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/embedded-content/tall-cross-domain-iframe-in-scrolled.sub-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html class="reftest-wait">
<iframe id=myframe style="width: 300px; height: 1000px" src="resources/tall.html"></iframe>
<div style="height: 2000px"></div>
<script>
  window.scrollTo(0, 700);
  myframe.onload = () => document.documentElement.classList.remove('reftest-wait');
</script>
</html>
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
  "source_name": "html/rendering/replaced-elements/embedded-content/tall-cross-domain-iframe-in-scrolled.sub-ref.html"
}
```
