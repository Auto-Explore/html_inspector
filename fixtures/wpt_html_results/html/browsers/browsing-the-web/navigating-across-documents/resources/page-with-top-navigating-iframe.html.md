# html/browsers/browsing-the-web/navigating-across-documents/resources/page-with-top-navigating-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/resources/page-with-top-navigating-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<script src=/common/get-host-info.sub.js></script>
<script src=/resources/testdriver.js></script>
<script src=/resources/testdriver-vendor.js></script>
<title>Page that embeds an iframe that navigates its top</title>
<script>
function addIframe() {
  const iframe = document.createElement('iframe');
  const path = new URL("top-navigating-page.html", window.location).pathname;
  iframe.src = get_host_info().HTTP_NOTSAMESITE_ORIGIN + path;
  document.body.appendChild(iframe);
}

addEventListener('load', () => {
  const urlParams = new URLSearchParams(location.search);
  const parentUserGesture = urlParams.get('parent_user_gesture') === 'true';
  if (parentUserGesture)
    test_driver.bless("Giving parent frame user activation").then(addIframe);
  else
    addIframe();
});
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/resources/page-with-top-navigating-iframe.html"
}
```
