# html/browsers/browsing-the-web/scroll-to-fragid/scroll-to-top.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/scroll-to-fragid/scroll-to-top.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Fragment Navigation: When fragid is TOP scroll to the top of the document</title>
<meta name=timeout content=long>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<div></div>
<div id="not-the-top"></div>
<div style="height:200em"></div>
<script>
var steps = [{
    fragid:'not-the-top',
    handler: function(){
      assert_not_equals( document.scrollingElement.scrollTop, 0 );
    }
  },{
    fragid:'top',
    handler: function(){
      assert_equals( document.scrollingElement.scrollTop, 0 );
    }
  },{
    fragid:'not-the-top',
    handler: function(){
      assert_not_equals( document.scrollingElement.scrollTop, 0 );
    }
  },{
    fragid:'TOP',
    handler: function(){
      assert_equals( document.scrollingElement.scrollTop, 0 );
    }
  }];

function runNextStep(){
    if( steps.length > 0 ) {
      var step = steps.shift();
      var listener = t.step_func( function(){
        step.handler();
        runNextStep();
      });
      scrollToFragmentThenDo( step.fragid, listener );
    } else {
      t.done();
    }
}

function scrollToFragmentThenDo( fragid, then ){
  location.hash = fragid;
  setTimeout( then, 1 );
}

var t = async_test();
t.step( function(){
  assert_equals(location.hash, "", "Page must be loaded with no hash");
  runNextStep();
})
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
  "source_name": "html/browsers/browsing-the-web/scroll-to-fragid/scroll-to-top.html"
}
```
