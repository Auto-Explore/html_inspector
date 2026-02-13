# html/syntax/parsing/unclosed-svg-script.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/unclosed-svg-script.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title></title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script>
    var scriptWithEndTagRan = false;
    var scriptWithoutEndTagRan = false;
    var scriptWithBogusEndTagInsideRan = false;
    var scriptWithBreakout = false;
    var scriptSelfClosing = false;
</script>
<svg>
    <script>scriptWithEndTagRan = true;</script>
</svg>
<svg>
    <script>scriptWithoutEndTagRan = true;
</svg>
<svg>
    <script>scriptWithBogusEndTagInsideRan = true;</g></script>
</svg>
<svg>
    <script>scriptWithBreakout = true;<s></script>
</svg>
<svg>
    <script href="support/svg-script-self-closing.js"/>
</svg>
</s>
<script>
    test(function() {
        assert_true(scriptWithEndTagRan);
    }, "SVG scripts with end tag should run");
    test(function() {
        assert_false(scriptWithoutEndTagRan);
    }, "SVG scripts without end tag should not run");
    test(function() {
        assert_true(scriptWithBogusEndTagInsideRan);
    }, "SVG scripts with bogus end tag inside should run");
    test(function() {
        assert_false(scriptWithBreakout);
    }, "SVG scripts ended by HTML breakout should not run");
    test(function() {
        assert_true(scriptSelfClosing);
    }, "SVG scripts with self-closing start tag should run");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.title.empty",
      "message": "Element “title” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 44,
        "byte_start": 37,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.parser.eof_in_rawtext",
      "message": "End of file seen when expecting text or an end tag.",
      "severity": "Error",
      "span": {
        "byte_end": 450,
        "byte_start": 442,
        "col": 5,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “g”.",
      "severity": "Error",
      "span": {
        "byte_end": 548,
        "byte_start": 544,
        "col": 51,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “s”.",
      "severity": "Error",
      "span": {
        "byte_end": 702,
        "byte_start": 698,
        "col": 1,
        "line": 28
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
  "source_name": "html/syntax/parsing/unclosed-svg-script.html"
}
```
