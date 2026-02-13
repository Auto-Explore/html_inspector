# html/browsers/browsing-the-web/navigating-across-documents/resources/top-navigating-page.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/resources/top-navigating-page.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Page that navigates its top</title>
<script src=/common/get-host-info.sub.js></script>
<script>

let path = new URL("page-that-post-message-to-opener.html", window.location).pathname;
let fullUrl = get_host_info().HTTP_NOTSAMESITE_ORIGIN + path;
try {
  top.location = fullUrl;
} catch {
  top.opener.postMessage('Denied', '*');
}

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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/resources/top-navigating-page.html"
}
```
