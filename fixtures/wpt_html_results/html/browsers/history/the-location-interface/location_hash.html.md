# html/browsers/history/the-location-interface/location_hash.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location_hash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
  <head>
    <title>location_hash</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <div id="log"></div>
    <iframe id="srcdoc-iframe"
       srcdoc="<div style='height: 200vh'></div><div id='test'></div>"></iframe>
    <script>
    function resetHash() {
      location.hash = "";
    }

    test(function (t) {
      t.add_cleanup(resetHash);
      window.history.pushState(1, document.title, '#x=1');
      var hash = location.hash;

      assert_equals(hash, "#x=1", "hash");

    }, "location hash");

    var t = async_test("Setting location.hash on srcdoc iframe");
    addEventListener("load", t.step_func_done(function() {
      var frameWin = document.getElementById("srcdoc-iframe").contentWindow;
      assert_equals(frameWin.location.href, "about:srcdoc");
      assert_equals(frameWin.scrollY, 0, "Should not have scrolled yet");
      frameWin.location.hash = "test";
      assert_equals(frameWin.location.href, "about:srcdoc#test");
      assert_true(frameWin.scrollY > frameWin.innerHeight,
                  "Should have scrolled by more than one viewport height");
    }));

    test(function(t) {
      t.add_cleanup(resetHash);
      location.hash = "test";
      assert_equals(location.hash, "#test");
    }, "Setting hash should automatically include hash character");

    test(function(t) {
      t.add_cleanup(resetHash);
      location.hash = "#not encoded";
      assert_equals(location.hash, "#not%20encoded");
    }, "Setting hash should encode incompatible characters");

    test(function(t) {
      t.add_cleanup(resetHash);
      location.hash = "#already%20encoded";
      assert_equals(location.hash, "#already%20encoded");
    }, "Setting hash to an already encoded value should not double encode it");

    test(function(t) {
      t.add_cleanup(resetHash);
      location.hash = "#mixed encoding%20here";
      assert_equals(location.hash, "#mixed%20encoding%20here");
    }, "Setting hash which is partially encoded should only encode incompatible characters");
    </script>
  </body>
</html>
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
  "source_name": "html/browsers/history/the-location-interface/location_hash.html"
}
```
