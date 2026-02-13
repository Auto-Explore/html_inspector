# html/links/manifest/no-manifest-from-iframe-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/links/manifest/no-manifest-from-iframe-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>
  Don't install manifests that are not top-level from browsing contexts
</title>
<link rel="help" href="https://html.spec.whatwg.org/#link-type-manifest" />
<h1>Don't install manifests that are not top-level from browsing contexts</h1>
<p>
  To pass, the user agent must not use the manifest in iframe. The user agent
  must behave as if there is no manifest present.
</p>
<script>
  const iframe = document.createElement("iframe");
  iframe.srcdoc = "<h1>hi</h1>";
  document.body.append(iframe);
  iframe.onload = () => {
    const link = iframe.contentDocument.createElement("link");
    link.rel = "manifest";
    link.href = "/appmanifest/name-member/name-member-fail.webmanifest";
    iframe.contentDocument.head.append(link);
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
  "source_name": "html/links/manifest/no-manifest-from-iframe-manual.html"
}
```
