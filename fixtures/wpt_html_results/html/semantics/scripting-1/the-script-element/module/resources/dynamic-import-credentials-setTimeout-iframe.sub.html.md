# html/semantics/scripting-1/the-script-element/module/resources/dynamic-import-credentials-setTimeout-iframe.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/resources/dynamic-import-credentials-setTimeout-iframe.sub.html",
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
The active script at the time of import() is the classic script created by
https://html.spec.whatwg.org/multipage/C/#timer-initialisation-steps
and the active script at the time of setTimeout() is the script elements below,
thus the credentials mode of the fetch options of the script elements below
are used for dynamic import requests.

setTimeout() calls below can't be wrapped (e.g. by step_timeout())
because wrapping setTimeout() would set active scripts differently.
-->

<script>
runTest(setTimeoutWrapper(x => setTimeout(x, 0)),
  "same", "found", "setTimeout(string) from classic script (crossOrigin not specified)");
runTest(setTimeoutWrapper(x => setTimeout(x, 0)),
  "cross", "not found", "setTimeout(string) from classic script (crossOrigin not specified)");
</script>

<script crossOrigin="anonymous">
runTest(setTimeoutWrapper(x => setTimeout(x, 0)),
  "same", "found", "setTimeout(string) from classic script (crossOrigin=anonymous)");
runTest(setTimeoutWrapper(x => setTimeout(x, 0)),
  "cross", "not found", "setTimeout(string) from classic script (crossOrigin=anonymous)");
</script>

<script crossOrigin="use-credentials">
runTest(setTimeoutWrapper(x => setTimeout(x, 0)),
  "same", "found", "setTimeout(string) from classic script (crossOrigin=use-credentials)");
runTest(setTimeoutWrapper(x => setTimeout(x, 0)),
  "cross", "found", "setTimeout(string) from classic script (crossOrigin=use-credentials)");
</script>

<script type="module">
runTest(setTimeoutWrapper(x => setTimeout(x, 0)),
  "same", "found", "setTimeout(string) from module script (crossOrigin not specified)");
runTest(setTimeoutWrapper(x => setTimeout(x, 0)),
  "cross", "not found", "setTimeout(string) from module script (crossOrigin not specified)");
</script>

<script type="module" crossOrigin="anonymous">
runTest(setTimeoutWrapper(x => setTimeout(x, 0)),
  "same", "found", "setTimeout(string) from module script (crossOrigin=anonymous)");
runTest(setTimeoutWrapper(x => setTimeout(x, 0)),
  "cross", "not found", "setTimeout(string) from module script (crossOrigin=anonymous)");
</script>

<script type="module" crossOrigin="use-credentials">
runTest(setTimeoutWrapper(x => setTimeout(x, 0)),
  "same", "found", "setTimeout(string) from module script (crossOrigin=use-credentials)");
runTest(setTimeoutWrapper(x => setTimeout(x, 0)),
  "cross", "found", "setTimeout(string) from module script (crossOrigin=use-credentials)");
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/resources/dynamic-import-credentials-setTimeout-iframe.sub.html"
}
```
