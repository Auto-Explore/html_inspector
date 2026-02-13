# html/browsers/browsing-the-web/overlapping-navigations-and-traversals/resources/nav-cancelation-2-helper.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/resources/nav-cancelation-2-helper.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Page with child frame that navigates slowly</title>

<!--
  This file is used by `../nav-cancelation-2.sub.html`. The iframe below is its
  grandchild iframe, and whenever its load event fires we report this up to our
  parent Document.
-->
<iframe src="slow.py"></iframe>

<script>
  window.parent.postMessage("grandchild frame created", "*");
  const iframe = document.querySelector('iframe');
  iframe.onload = e => {
    window.parent.postMessage("grandchild frame loaded", "*");
  };
</script>
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
  "source_name": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/resources/nav-cancelation-2-helper.html"
}
```
