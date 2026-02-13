# html/semantics/scripting-1/the-script-element/async_011.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/async_011.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
    <head>
        <title>An empty parser-inserted script element should return async=true</title>
        <meta http-equiv="content-type" content="text/html; charset=UTF-8" />
        <meta description="An empty parser-inserted script element should return async=true." />
        <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
        <link rel="help" href="https://html.spec.whatwg.org/multipage/#prepare-a-script"/>
        <script src="/resources/testharness.js"></script>
        <script src="/resources/testharnessreport.js"></script>
    </head>
    <body>
        <div id=log></div>
        <script></script>
        <script type="text/javascript">
            test(function() {   assert_true(document.getElementsByTagName("script")[2].async)}, "An empty parser-inserted script element should return async=true");
        </script>
    </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 706,
        "byte_start": 675,
        "col": 9,
        "line": 15
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
  "source_name": "html/semantics/scripting-1/the-script-element/async_011.htm"
}
```
