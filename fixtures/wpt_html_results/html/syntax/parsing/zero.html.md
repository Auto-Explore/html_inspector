# html/syntax/parsing/zero.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/zero.html",
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
<body>
<div></div>
<script>
    test(function() {
        var div = document.getElementsByTagName("div")[0];
        div.innerHTML = "<!-\u0000>";
        assert_equals(div.firstChild.data, "-\uFFFD");
    }, "U+0000 should get replaced with U+FFFD after markup declaration hyphen");



    test(function() {
        var div = document.getElementsByTagName("div")[0];
        div.innerHTML = "&\u0000auml;";
        assert_equals(div.firstChild.data, "&auml;");
    }, "U+0000 should vanish after ampersand");

    test(function() {
        var div = document.getElementsByTagName("div")[0];
        div.innerHTML = "&a\u0000uml;";
        assert_equals(div.firstChild.data, "&auml;");
    }, "U+0000 should vanish after ampersand and one letter of entity prefix");

    test(function() {
        var div = document.getElementsByTagName("div")[0];
        div.innerHTML = "&au\u0000ml;";
        assert_equals(div.firstChild.data, "&auml;");
    }, "U+0000 should vanish after ampersand and two letters of entity prefix");

    test(function() {
        var div = document.getElementsByTagName("div")[0];
        div.innerHTML = "&aum\u0000l;";
        assert_equals(div.firstChild.data, "&auml;");
    }, "U+0000 should vanish after ampersand and three letters of entity prefix");

    test(function() {
        var div = document.getElementsByTagName("div")[0];
        div.innerHTML = "&auml\u0000;";
        assert_equals(div.firstChild.data, "\u00E4;");
    }, "U+0000 should vanish after semicolonless entity");

    test(function() {
        var div = document.getElementsByTagName("div")[0];
        div.innerHTML = "&notin\u0000;";
        assert_equals(div.firstChild.data, "\u00ACin;");
    }, "U+0000 should vanish before required semicolon");



    test(function() {
        var div = document.getElementsByTagName("div")[0];
        div.innerHTML = "a\u0000b";
        assert_equals(div.firstChild.data, "ab");
    }, "U+0000 should be dropped in body");



    test(function() {
        var div = document.getElementsByTagName("div")[0];
        div.innerHTML = "<span title='&\u0000auml;'>";
        assert_equals(div.firstChild.title, "&\uFFFDauml;");
    }, "U+0000 should get replaced with U+FFFD after ampersand");

    test(function() {
        var div = document.getElementsByTagName("div")[0];
        div.innerHTML = "<span title='&a\u0000uml;'>";
        assert_equals(div.firstChild.title, "&a\uFFFDuml;");
    }, "U+0000 should get replaced with U+FFFD after ampersand and one letter of entity prefix");

    test(function() {
        var div = document.getElementsByTagName("div")[0];
        div.innerHTML = "<span title='&au\u0000ml;'>";
        assert_equals(div.firstChild.title, "&au\uFFFDml;");
    }, "U+0000 should get replaced with U+FFFD after ampersand and two letters of entity prefix");

    test(function() {
        var div = document.getElementsByTagName("div")[0];
        div.innerHTML = "<span title='&aum\u0000l;'>";
        assert_equals(div.firstChild.title, "&aum\uFFFDl;");
    }, "U+0000 should get replaced with U+FFFD after ampersand and three letters of entity prefix");

    test(function() {
        var div = document.getElementsByTagName("div")[0];
        div.innerHTML = "<span title='&auml\u0000;'>";
        assert_equals(div.firstChild.title, "\u00E4\uFFFD;");
    }, "U+0000 should get replaced with U+FFFD after semicolonless entity");

    test(function() {
        var div = document.getElementsByTagName("div")[0];
        div.innerHTML = "<span title='&notin\u0000;'>";
        assert_equals(div.firstChild.title, "&notin\uFFFD;");
    }, "U+0000 should get replaced with U+FFFD before required semicolon");



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
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/syntax/parsing/zero.html"
}
```
