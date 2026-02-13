# html/semantics/scripting-1/the-script-element/module/resources/dynamic-import-credentials-iframe.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/resources/dynamic-import-credentials-iframe.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="dynamic-import-credentials-helper.sub.js"></script>

<!--
The active script at the time of import() is the script elements below, and
thus the credentials mode of the fetch options of the script elements below
are used for dynamic import requests.
-->

<script>
runTest(url => import(url),
  "same", "found", "classic script (crossOrigin not specified)");
runTest(url => import(url),
  "cross", "not found", "classic script (crossOrigin not specified)");
</script>

<script crossOrigin="anonymous">
runTest(url => import(url), "same", "found",
  "classic script (crossOrigin=anonymous)");
runTest(url => import(url), "cross", "not found",
  "classic script (crossOrigin=anonymous)");
</script>

<script crossOrigin="use-credentials">
runTest(url => import(url),
  "same", "found", "classic script (crossOrigin=use-credentials)");
runTest(url => import(url),
  "cross", "found", "classic script (crossOrigin=use-credentials)");
</script>

<script type="module">
runTest(url => import(url),
  "same", "found", "module script (crossOrigin not specified)");
runTest(url => import(url),
  "cross", "not found", "module script (crossOrigin not specified)");
</script>

<script type="module" crossOrigin="anonymous">
runTest(url => import(url), "same", "found",
  "module script (crossOrigin=anonymous)");
runTest(url => import(url), "cross", "not found",
  "module script (crossOrigin=anonymous)");
</script>

<script type="module" crossOrigin="use-credentials">
runTest(url => import(url),
  "same", "found", "module script (crossOrigin=use-credentials)");
runTest(url => import(url),
  "cross", "found", "module script (crossOrigin=use-credentials)");
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/resources/dynamic-import-credentials-iframe.sub.html"
}
```
