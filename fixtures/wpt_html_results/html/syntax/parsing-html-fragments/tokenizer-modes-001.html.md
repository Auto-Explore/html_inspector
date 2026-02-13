# html/syntax/parsing-html-fragments/tokenizer-modes-001.html

Counts:
- errors: 0
- warnings: 0
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing-html-fragments/tokenizer-modes-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html lang="en" >
<head>
  <title>Tokenizer states</title>
<link rel='author' title='Henri Sivonen' href='mailto:hsivonen@hsivonen.fi'>
<link rel='help' href='https://html.spec.whatwg.org/#html-fragment-parsing-algorithm'>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<script>
test(function() {
  var e = document.createElement("title");
  e.innerHTML = "</title><div>";
  assert_equals(e.getElementsByTagName("div").length, 0);
  assert_equals(e.innerHTML, "&lt;/title&gt;&lt;div&gt;");
}, "</title> should not break out of title.");

test(function() {
  var e = document.createElement("textarea");
  e.innerHTML = "</textarea><div>";
  assert_equals(e.getElementsByTagName("div").length, 0);
  assert_equals(e.innerHTML, "&lt;/textarea&gt;&lt;div&gt;");
}, "</textarea> should not break out of textarea.");

test(function() {
  var e = document.createElement("style");
  e.innerHTML = "</style><div>";
  assert_equals(e.getElementsByTagName("div").length, 0);
  assert_equals(e.innerHTML, "</style><div>");
}, "</style> should not break out of style.");

test(function() {
  var e = document.createElement("xmp");
  e.innerHTML = "</xmp><div>";
  assert_equals(e.getElementsByTagName("div").length, 0);
  assert_equals(e.innerHTML, "</xmp><div>");
}, "</xmp> should not break out of xmp.");

test(function() {
  var e = document.createElement("iframe");
  e.innerHTML = "</iframe><div>";
  assert_equals(e.getElementsByTagName("div").length, 0);
  assert_equals(e.innerHTML, "</iframe><div>");
}, "</iframe> should not break out of iframe.");

test(function() {
  var e = document.createElement("noembed");
  e.innerHTML = "</noembed><div>";
  assert_equals(e.getElementsByTagName("div").length, 0);
  assert_equals(e.innerHTML, "</noembed><div>");
}, "</noembed> should not break out of noembed.");

test(function() {
  var e = document.createElement("noframes");
  e.innerHTML = "</noframes><div>";
  assert_equals(e.getElementsByTagName("div").length, 0);
  assert_equals(e.innerHTML, "</noframes><div>");
}, "</noframes> should not break out of noframes.");

test(function() {
  var e = document.createElement("script");
  e.innerHTML = "<\/script><div>";
  assert_equals(e.getElementsByTagName("div").length, 0);
  assert_equals(e.innerHTML, "<\/script><div>");
}, "<\/script> should not break out of script.");


test(function() {
  var e = document.createElement("noscript");
  e.innerHTML = "</noscript><div>";
  assert_equals(e.getElementsByTagName("div").length, 0);
  assert_equals(e.innerHTML, "</noscript><div>");
}, "</noscript> should not break out of noscript.");

test(function() {
  var e = document.createElement("plaintext");
  e.innerHTML = "</plaintext><div>";
  assert_equals(e.getElementsByTagName("div").length, 0);
  assert_equals(e.innerHTML, "</plaintext><div>");
}, "</plaintext> should not break out of plaintext.");

</script>
</body>
</html>
```

```json
{
  "messages": [],
  "source_name": "html/syntax/parsing-html-fragments/tokenizer-modes-001.html"
}
```
