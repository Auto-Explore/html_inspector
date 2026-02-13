# html/scripting/the-script-element/attr-script-fetchpriority.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/scripting/the-script-element/attr-script-fetchpriority.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Fetch Priority - Script element</title>
<meta name="author" title="Patrick Meenan" href="mailto:patmeenan@gmail.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script id=script1 src="resources/script.js" fetchpriority=high></script>
<script id=script2 src="resources/script.js" fetchpriority=low></script>
<script id=script3 src="resources/script.js" fetchpriority=auto></script>
<script id=script4 src="resources/script.js" fetchpriority=xyz></script>
<script id=script5 src="resources/script.js"></script>

<script>
  test(() => {
    assert_equals(script1.fetchPriority, "high", "high fetchPriority is a valid IDL value on the script element");
    assert_equals(script2.fetchPriority, "low", "low fetchPriority is a valid IDL value on the script element");
    assert_equals(script3.fetchPriority, "auto", "auto fetchPriority is a valid IDL value on the script element");
    assert_equals(script4.fetchPriority, "auto", "invalid fetchPriority reflects as 'auto' IDL attribute on the script element");
    assert_equals(script5.fetchPriority, "auto", "missing fetchPriority reflects as 'auto' IDL attribute on the script element");
  }, "fetchpriority attribute on <script> elements should reflect valid IDL values");

  test(() => {
    const script = document.createElement("script");
    assert_equals(script.fetchPriority, "auto");
  }, "default fetchpriority attribute on <script> elements should be 'auto'");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 140,
        "byte_start": 63,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 140,
        "byte_start": 63,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/scripting/the-script-element/attr-script-fetchpriority.html"
}
```
