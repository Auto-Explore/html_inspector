# html/semantics/scripting-1/the-script-element/load-error-events-3.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/load-error-events-3.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<head>
<title>load/error events for classic scripts with a style sheet that is blocking scripts and script nesting level &gt; 1</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/load-error-events-helpers.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#execute-the-script-block">
</head>
<script>
"use strict";
var test6_load = event_test('no src, parser-inserted, has style sheets blocking scripts, script nesting level == 2',
    false, false);

document.write(
    `<link rel="stylesheet" href="/common/slow.py"></link>
    <script onload="onLoad(test6_load);"
        onerror="onError(test6_load);">
    "use strict";
    onExecute(test6_load);
    </scr` + `ipt>`);
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
  "source_name": "html/semantics/scripting-1/the-script-element/load-error-events-3.html"
}
```
