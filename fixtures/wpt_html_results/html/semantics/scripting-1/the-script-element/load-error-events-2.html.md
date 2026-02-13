# html/semantics/scripting-1/the-script-element/load-error-events-2.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/load-error-events-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<head>
<title>load/error events for classic scripts with a style sheet that is blocking scripts</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/load-error-events-helpers.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#execute-the-script-block">
<script>
"use strict";
var test5_load = event_test('no src, parser-inserted, has style sheets blocking scripts, script nesting level == 1', false, false);
</script>

<link rel="stylesheet" href="/common/slow.py"></link>
<!-- This is testing the case where an inline classic script is inserted
by parser while there is an loading stylesheet. Therefore, it is critical to
place a <link rel="stylesheet"> just above the <script> to be tested. -->
<script onload="onLoad(test5_load);" onerror="onError(test5_load);">
"use strict";
onExecute(test5_load);
</script>
</head>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “link”.",
      "severity": "Error",
      "span": {
        "byte_end": 621,
        "byte_start": 614,
        "col": 47,
        "line": 14
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
  "source_name": "html/semantics/scripting-1/the-script-element/load-error-events-2.html"
}
```
