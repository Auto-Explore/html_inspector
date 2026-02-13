# html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLMediaElement/addTextTrack.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLMediaElement/addTextTrack.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>HTMLMediaElement.addTextTrack</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
var video = document.createElement('video');
test(function(){
    assert_throws_js(TypeError, function(){
        video.addTextTrack('foo');
    });
    assert_throws_js(TypeError, function(){
        video.addTextTrack(undefined);
    });
    assert_throws_js(TypeError, function(){
        video.addTextTrack(null);
    });
}, document.title + ' bogus first arg');

test(function(){
    assert_throws_js(TypeError, function(){
        video.addTextTrack('SUBTITLES');
    });
}, document.title + ' uppercase first arg');

test(function(){
    var t = video.addTextTrack('subtitles');
    assert_equals(t.kind, 'subtitles');
    assert_equals(t.label, '');
    assert_equals(t.language, '');
    assert_equals(t.mode, 'hidden');
    assert_true(t.cues instanceof TextTrackCueList);
    assert_equals(t.cues.length, 0);
}, document.title + ' subtitles first arg');

test(function(){
    var t = video.addTextTrack('captions');
    assert_equals(t.kind, 'captions');
    assert_equals(t.label, '');
    assert_equals(t.language, '');
    assert_equals(t.mode, 'hidden');
    assert_true(t.cues instanceof TextTrackCueList);
    assert_equals(t.cues.length, 0);
}, document.title + ' captions first arg');

test(function(){
    var t = video.addTextTrack('descriptions');
    assert_equals(t.kind, 'descriptions');
    assert_equals(t.label, '');
    assert_equals(t.language, '');
    assert_equals(t.mode, 'hidden');
    assert_true(t.cues instanceof TextTrackCueList);
    assert_equals(t.cues.length, 0);
}, document.title + ' descriptions first arg');

test(function(){
    var t = video.addTextTrack('chapters');
    assert_equals(t.kind, 'chapters');
    assert_equals(t.label, '');
    assert_equals(t.language, '');
    assert_equals(t.mode, 'hidden');
    assert_true(t.cues instanceof TextTrackCueList);
    assert_equals(t.cues.length, 0);
}, document.title + ' chapters first arg');

test(function(){
    var t = video.addTextTrack('metadata');
    assert_equals(t.kind, 'metadata');
    assert_equals(t.label, '');
    assert_equals(t.language, '');
    assert_equals(t.mode, 'hidden');
    assert_true(t.cues instanceof TextTrackCueList);
    assert_equals(t.cues.length, 0);
}, document.title + ' metadata first arg');

test(function(){
    var t = video.addTextTrack('subtitles', undefined, undefined);
    assert_equals(t.kind, 'subtitles');
    assert_equals(t.label, '');
    assert_equals(t.language, '');
    assert_equals(t.mode, 'hidden');
    assert_true(t.cues instanceof TextTrackCueList);
    assert_equals(t.cues.length, 0);
}, document.title + ' undefined second and third arg');

test(function(){
    var t = video.addTextTrack('subtitles', null, null);
    assert_equals(t.kind, 'subtitles');
    assert_equals(t.label, 'null');
    assert_equals(t.language, 'null');
    assert_equals(t.mode, 'hidden');
    assert_true(t.cues instanceof TextTrackCueList);
    assert_equals(t.cues.length, 0);
}, document.title + ' null second and third arg');

test(function(){
    var t = video.addTextTrack('subtitles', 'foo', 'bar');
    assert_equals(t.kind, 'subtitles');
    assert_equals(t.label, 'foo');
    assert_equals(t.language, 'bar');
    assert_equals(t.mode, 'hidden');
    assert_true(t.cues instanceof TextTrackCueList);
    assert_equals(t.cues.length, 0);
}, document.title + ' foo and bar second and third arg');

test(function(){
    var t = video.addTextTrack('subtitles', 'foo');
    assert_equals(t.kind, 'subtitles');
    assert_equals(t.label, 'foo');
    assert_equals(t.language, '');
    assert_equals(t.mode, 'hidden');
    assert_true(t.cues instanceof TextTrackCueList);
    assert_equals(t.cues.length, 0);
}, document.title + ' foo second arg, third arg omitted');

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
  "source_name": "html/semantics/embedded-content/media-elements/interfaces/HTMLElement/HTMLMediaElement/addTextTrack.html"
}
```
