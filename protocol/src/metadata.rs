// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct TopTracks {
    // message fields
    country: ::protobuf::SingularField<::std::string::String>,
    track: ::protobuf::RepeatedField<Track>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TopTracks {}

impl TopTracks {
    pub fn new() -> TopTracks {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TopTracks {
        static mut instance: ::protobuf::lazy::Lazy<TopTracks> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TopTracks,
        };
        unsafe {
            instance.get(|| {
                TopTracks {
                    country: ::protobuf::SingularField::none(),
                    track: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string country = 1;

    pub fn clear_country(&mut self) {
        self.country.clear();
    }

    pub fn has_country(&self) -> bool {
        self.country.is_some()
    }

    // Param is passed by value, moved
    pub fn set_country(&mut self, v: ::std::string::String) {
        self.country = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_country(&mut self) -> &mut ::std::string::String {
        if self.country.is_none() {
            self.country.set_default();
        };
        self.country.as_mut().unwrap()
    }

    // Take field
    pub fn take_country(&mut self) -> ::std::string::String {
        self.country.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_country(&self) -> &str {
        match self.country.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .Track track = 2;

    pub fn clear_track(&mut self) {
        self.track.clear();
    }

    // Param is passed by value, moved
    pub fn set_track(&mut self, v: ::protobuf::RepeatedField<Track>) {
        self.track = v;
    }

    // Mutable pointer to the field.
    pub fn mut_track(&mut self) -> &mut ::protobuf::RepeatedField<Track> {
        &mut self.track
    }

    // Take field
    pub fn take_track(&mut self) -> ::protobuf::RepeatedField<Track> {
        ::std::mem::replace(&mut self.track, ::protobuf::RepeatedField::new())
    }

    pub fn get_track(&self) -> &[Track] {
        &self.track
    }
}

impl ::protobuf::Message for TopTracks {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.country));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.track));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.country {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.track {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.country.as_ref() {
            try!(os.write_string(1, &v));
        };
        for v in &self.track {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TopTracks>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TopTracks {
    fn new() -> TopTracks {
        TopTracks::new()
    }

    fn descriptor_static(_: ::std::option::Option<TopTracks>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "country",
                    TopTracks::has_country,
                    TopTracks::get_country,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "track",
                    TopTracks::get_track,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TopTracks>(
                    "TopTracks",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TopTracks {
    fn clear(&mut self) {
        self.clear_country();
        self.clear_track();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TopTracks {
    fn eq(&self, other: &TopTracks) -> bool {
        self.country == other.country &&
        self.track == other.track &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TopTracks {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ActivityPeriod {
    // message fields
    start_year: ::std::option::Option<i32>,
    end_year: ::std::option::Option<i32>,
    decade: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActivityPeriod {}

impl ActivityPeriod {
    pub fn new() -> ActivityPeriod {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActivityPeriod {
        static mut instance: ::protobuf::lazy::Lazy<ActivityPeriod> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActivityPeriod,
        };
        unsafe {
            instance.get(|| {
                ActivityPeriod {
                    start_year: ::std::option::Option::None,
                    end_year: ::std::option::Option::None,
                    decade: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional sint32 start_year = 1;

    pub fn clear_start_year(&mut self) {
        self.start_year = ::std::option::Option::None;
    }

    pub fn has_start_year(&self) -> bool {
        self.start_year.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_year(&mut self, v: i32) {
        self.start_year = ::std::option::Option::Some(v);
    }

    pub fn get_start_year(&self) -> i32 {
        self.start_year.unwrap_or(0)
    }

    // optional sint32 end_year = 2;

    pub fn clear_end_year(&mut self) {
        self.end_year = ::std::option::Option::None;
    }

    pub fn has_end_year(&self) -> bool {
        self.end_year.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_year(&mut self, v: i32) {
        self.end_year = ::std::option::Option::Some(v);
    }

    pub fn get_end_year(&self) -> i32 {
        self.end_year.unwrap_or(0)
    }

    // optional sint32 decade = 3;

    pub fn clear_decade(&mut self) {
        self.decade = ::std::option::Option::None;
    }

    pub fn has_decade(&self) -> bool {
        self.decade.is_some()
    }

    // Param is passed by value, moved
    pub fn set_decade(&mut self, v: i32) {
        self.decade = ::std::option::Option::Some(v);
    }

    pub fn get_decade(&self) -> i32 {
        self.decade.unwrap_or(0)
    }
}

impl ::protobuf::Message for ActivityPeriod {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.start_year = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.end_year = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.decade = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.start_year {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, *value);
        };
        for value in &self.end_year {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, *value);
        };
        for value in &self.decade {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_year {
            try!(os.write_sint32(1, v));
        };
        if let Some(v) = self.end_year {
            try!(os.write_sint32(2, v));
        };
        if let Some(v) = self.decade {
            try!(os.write_sint32(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ActivityPeriod>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ActivityPeriod {
    fn new() -> ActivityPeriod {
        ActivityPeriod::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActivityPeriod>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "start_year",
                    ActivityPeriod::has_start_year,
                    ActivityPeriod::get_start_year,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "end_year",
                    ActivityPeriod::has_end_year,
                    ActivityPeriod::get_end_year,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "decade",
                    ActivityPeriod::has_decade,
                    ActivityPeriod::get_decade,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActivityPeriod>(
                    "ActivityPeriod",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActivityPeriod {
    fn clear(&mut self) {
        self.clear_start_year();
        self.clear_end_year();
        self.clear_decade();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ActivityPeriod {
    fn eq(&self, other: &ActivityPeriod) -> bool {
        self.start_year == other.start_year &&
        self.end_year == other.end_year &&
        self.decade == other.decade &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ActivityPeriod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Artist {
    // message fields
    gid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    name: ::protobuf::SingularField<::std::string::String>,
    popularity: ::std::option::Option<i32>,
    top_track: ::protobuf::RepeatedField<TopTracks>,
    album_group: ::protobuf::RepeatedField<AlbumGroup>,
    single_group: ::protobuf::RepeatedField<AlbumGroup>,
    compilation_group: ::protobuf::RepeatedField<AlbumGroup>,
    appears_on_group: ::protobuf::RepeatedField<AlbumGroup>,
    genre: ::protobuf::RepeatedField<::std::string::String>,
    external_id: ::protobuf::RepeatedField<ExternalId>,
    portrait: ::protobuf::RepeatedField<Image>,
    biography: ::protobuf::RepeatedField<Biography>,
    activity_period: ::protobuf::RepeatedField<ActivityPeriod>,
    restriction: ::protobuf::RepeatedField<Restriction>,
    related: ::protobuf::RepeatedField<Artist>,
    is_portrait_album_cover: ::std::option::Option<bool>,
    portrait_group: ::protobuf::SingularPtrField<ImageGroup>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Artist {}

impl Artist {
    pub fn new() -> Artist {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Artist {
        static mut instance: ::protobuf::lazy::Lazy<Artist> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Artist,
        };
        unsafe {
            instance.get(|| {
                Artist {
                    gid: ::protobuf::SingularField::none(),
                    name: ::protobuf::SingularField::none(),
                    popularity: ::std::option::Option::None,
                    top_track: ::protobuf::RepeatedField::new(),
                    album_group: ::protobuf::RepeatedField::new(),
                    single_group: ::protobuf::RepeatedField::new(),
                    compilation_group: ::protobuf::RepeatedField::new(),
                    appears_on_group: ::protobuf::RepeatedField::new(),
                    genre: ::protobuf::RepeatedField::new(),
                    external_id: ::protobuf::RepeatedField::new(),
                    portrait: ::protobuf::RepeatedField::new(),
                    biography: ::protobuf::RepeatedField::new(),
                    activity_period: ::protobuf::RepeatedField::new(),
                    restriction: ::protobuf::RepeatedField::new(),
                    related: ::protobuf::RepeatedField::new(),
                    is_portrait_album_cover: ::std::option::Option::None,
                    portrait_group: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes gid = 1;

    pub fn clear_gid(&mut self) {
        self.gid.clear();
    }

    pub fn has_gid(&self) -> bool {
        self.gid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gid(&mut self, v: ::std::vec::Vec<u8>) {
        self.gid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.gid.is_none() {
            self.gid.set_default();
        };
        self.gid.as_mut().unwrap()
    }

    // Take field
    pub fn take_gid(&mut self) -> ::std::vec::Vec<u8> {
        self.gid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_gid(&self) -> &[u8] {
        match self.gid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional sint32 popularity = 3;

    pub fn clear_popularity(&mut self) {
        self.popularity = ::std::option::Option::None;
    }

    pub fn has_popularity(&self) -> bool {
        self.popularity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_popularity(&mut self, v: i32) {
        self.popularity = ::std::option::Option::Some(v);
    }

    pub fn get_popularity(&self) -> i32 {
        self.popularity.unwrap_or(0)
    }

    // repeated .TopTracks top_track = 4;

    pub fn clear_top_track(&mut self) {
        self.top_track.clear();
    }

    // Param is passed by value, moved
    pub fn set_top_track(&mut self, v: ::protobuf::RepeatedField<TopTracks>) {
        self.top_track = v;
    }

    // Mutable pointer to the field.
    pub fn mut_top_track(&mut self) -> &mut ::protobuf::RepeatedField<TopTracks> {
        &mut self.top_track
    }

    // Take field
    pub fn take_top_track(&mut self) -> ::protobuf::RepeatedField<TopTracks> {
        ::std::mem::replace(&mut self.top_track, ::protobuf::RepeatedField::new())
    }

    pub fn get_top_track(&self) -> &[TopTracks] {
        &self.top_track
    }

    // repeated .AlbumGroup album_group = 5;

    pub fn clear_album_group(&mut self) {
        self.album_group.clear();
    }

    // Param is passed by value, moved
    pub fn set_album_group(&mut self, v: ::protobuf::RepeatedField<AlbumGroup>) {
        self.album_group = v;
    }

    // Mutable pointer to the field.
    pub fn mut_album_group(&mut self) -> &mut ::protobuf::RepeatedField<AlbumGroup> {
        &mut self.album_group
    }

    // Take field
    pub fn take_album_group(&mut self) -> ::protobuf::RepeatedField<AlbumGroup> {
        ::std::mem::replace(&mut self.album_group, ::protobuf::RepeatedField::new())
    }

    pub fn get_album_group(&self) -> &[AlbumGroup] {
        &self.album_group
    }

    // repeated .AlbumGroup single_group = 6;

    pub fn clear_single_group(&mut self) {
        self.single_group.clear();
    }

    // Param is passed by value, moved
    pub fn set_single_group(&mut self, v: ::protobuf::RepeatedField<AlbumGroup>) {
        self.single_group = v;
    }

    // Mutable pointer to the field.
    pub fn mut_single_group(&mut self) -> &mut ::protobuf::RepeatedField<AlbumGroup> {
        &mut self.single_group
    }

    // Take field
    pub fn take_single_group(&mut self) -> ::protobuf::RepeatedField<AlbumGroup> {
        ::std::mem::replace(&mut self.single_group, ::protobuf::RepeatedField::new())
    }

    pub fn get_single_group(&self) -> &[AlbumGroup] {
        &self.single_group
    }

    // repeated .AlbumGroup compilation_group = 7;

    pub fn clear_compilation_group(&mut self) {
        self.compilation_group.clear();
    }

    // Param is passed by value, moved
    pub fn set_compilation_group(&mut self, v: ::protobuf::RepeatedField<AlbumGroup>) {
        self.compilation_group = v;
    }

    // Mutable pointer to the field.
    pub fn mut_compilation_group(&mut self) -> &mut ::protobuf::RepeatedField<AlbumGroup> {
        &mut self.compilation_group
    }

    // Take field
    pub fn take_compilation_group(&mut self) -> ::protobuf::RepeatedField<AlbumGroup> {
        ::std::mem::replace(&mut self.compilation_group, ::protobuf::RepeatedField::new())
    }

    pub fn get_compilation_group(&self) -> &[AlbumGroup] {
        &self.compilation_group
    }

    // repeated .AlbumGroup appears_on_group = 8;

    pub fn clear_appears_on_group(&mut self) {
        self.appears_on_group.clear();
    }

    // Param is passed by value, moved
    pub fn set_appears_on_group(&mut self, v: ::protobuf::RepeatedField<AlbumGroup>) {
        self.appears_on_group = v;
    }

    // Mutable pointer to the field.
    pub fn mut_appears_on_group(&mut self) -> &mut ::protobuf::RepeatedField<AlbumGroup> {
        &mut self.appears_on_group
    }

    // Take field
    pub fn take_appears_on_group(&mut self) -> ::protobuf::RepeatedField<AlbumGroup> {
        ::std::mem::replace(&mut self.appears_on_group, ::protobuf::RepeatedField::new())
    }

    pub fn get_appears_on_group(&self) -> &[AlbumGroup] {
        &self.appears_on_group
    }

    // repeated string genre = 9;

    pub fn clear_genre(&mut self) {
        self.genre.clear();
    }

    // Param is passed by value, moved
    pub fn set_genre(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.genre = v;
    }

    // Mutable pointer to the field.
    pub fn mut_genre(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.genre
    }

    // Take field
    pub fn take_genre(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.genre, ::protobuf::RepeatedField::new())
    }

    pub fn get_genre(&self) -> &[::std::string::String] {
        &self.genre
    }

    // repeated .ExternalId external_id = 10;

    pub fn clear_external_id(&mut self) {
        self.external_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_external_id(&mut self, v: ::protobuf::RepeatedField<ExternalId>) {
        self.external_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_external_id(&mut self) -> &mut ::protobuf::RepeatedField<ExternalId> {
        &mut self.external_id
    }

    // Take field
    pub fn take_external_id(&mut self) -> ::protobuf::RepeatedField<ExternalId> {
        ::std::mem::replace(&mut self.external_id, ::protobuf::RepeatedField::new())
    }

    pub fn get_external_id(&self) -> &[ExternalId] {
        &self.external_id
    }

    // repeated .Image portrait = 11;

    pub fn clear_portrait(&mut self) {
        self.portrait.clear();
    }

    // Param is passed by value, moved
    pub fn set_portrait(&mut self, v: ::protobuf::RepeatedField<Image>) {
        self.portrait = v;
    }

    // Mutable pointer to the field.
    pub fn mut_portrait(&mut self) -> &mut ::protobuf::RepeatedField<Image> {
        &mut self.portrait
    }

    // Take field
    pub fn take_portrait(&mut self) -> ::protobuf::RepeatedField<Image> {
        ::std::mem::replace(&mut self.portrait, ::protobuf::RepeatedField::new())
    }

    pub fn get_portrait(&self) -> &[Image] {
        &self.portrait
    }

    // repeated .Biography biography = 12;

    pub fn clear_biography(&mut self) {
        self.biography.clear();
    }

    // Param is passed by value, moved
    pub fn set_biography(&mut self, v: ::protobuf::RepeatedField<Biography>) {
        self.biography = v;
    }

    // Mutable pointer to the field.
    pub fn mut_biography(&mut self) -> &mut ::protobuf::RepeatedField<Biography> {
        &mut self.biography
    }

    // Take field
    pub fn take_biography(&mut self) -> ::protobuf::RepeatedField<Biography> {
        ::std::mem::replace(&mut self.biography, ::protobuf::RepeatedField::new())
    }

    pub fn get_biography(&self) -> &[Biography] {
        &self.biography
    }

    // repeated .ActivityPeriod activity_period = 13;

    pub fn clear_activity_period(&mut self) {
        self.activity_period.clear();
    }

    // Param is passed by value, moved
    pub fn set_activity_period(&mut self, v: ::protobuf::RepeatedField<ActivityPeriod>) {
        self.activity_period = v;
    }

    // Mutable pointer to the field.
    pub fn mut_activity_period(&mut self) -> &mut ::protobuf::RepeatedField<ActivityPeriod> {
        &mut self.activity_period
    }

    // Take field
    pub fn take_activity_period(&mut self) -> ::protobuf::RepeatedField<ActivityPeriod> {
        ::std::mem::replace(&mut self.activity_period, ::protobuf::RepeatedField::new())
    }

    pub fn get_activity_period(&self) -> &[ActivityPeriod] {
        &self.activity_period
    }

    // repeated .Restriction restriction = 14;

    pub fn clear_restriction(&mut self) {
        self.restriction.clear();
    }

    // Param is passed by value, moved
    pub fn set_restriction(&mut self, v: ::protobuf::RepeatedField<Restriction>) {
        self.restriction = v;
    }

    // Mutable pointer to the field.
    pub fn mut_restriction(&mut self) -> &mut ::protobuf::RepeatedField<Restriction> {
        &mut self.restriction
    }

    // Take field
    pub fn take_restriction(&mut self) -> ::protobuf::RepeatedField<Restriction> {
        ::std::mem::replace(&mut self.restriction, ::protobuf::RepeatedField::new())
    }

    pub fn get_restriction(&self) -> &[Restriction] {
        &self.restriction
    }

    // repeated .Artist related = 15;

    pub fn clear_related(&mut self) {
        self.related.clear();
    }

    // Param is passed by value, moved
    pub fn set_related(&mut self, v: ::protobuf::RepeatedField<Artist>) {
        self.related = v;
    }

    // Mutable pointer to the field.
    pub fn mut_related(&mut self) -> &mut ::protobuf::RepeatedField<Artist> {
        &mut self.related
    }

    // Take field
    pub fn take_related(&mut self) -> ::protobuf::RepeatedField<Artist> {
        ::std::mem::replace(&mut self.related, ::protobuf::RepeatedField::new())
    }

    pub fn get_related(&self) -> &[Artist] {
        &self.related
    }

    // optional bool is_portrait_album_cover = 16;

    pub fn clear_is_portrait_album_cover(&mut self) {
        self.is_portrait_album_cover = ::std::option::Option::None;
    }

    pub fn has_is_portrait_album_cover(&self) -> bool {
        self.is_portrait_album_cover.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_portrait_album_cover(&mut self, v: bool) {
        self.is_portrait_album_cover = ::std::option::Option::Some(v);
    }

    pub fn get_is_portrait_album_cover(&self) -> bool {
        self.is_portrait_album_cover.unwrap_or(false)
    }

    // optional .ImageGroup portrait_group = 17;

    pub fn clear_portrait_group(&mut self) {
        self.portrait_group.clear();
    }

    pub fn has_portrait_group(&self) -> bool {
        self.portrait_group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_portrait_group(&mut self, v: ImageGroup) {
        self.portrait_group = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_portrait_group(&mut self) -> &mut ImageGroup {
        if self.portrait_group.is_none() {
            self.portrait_group.set_default();
        };
        self.portrait_group.as_mut().unwrap()
    }

    // Take field
    pub fn take_portrait_group(&mut self) -> ImageGroup {
        self.portrait_group.take().unwrap_or_else(|| ImageGroup::new())
    }

    pub fn get_portrait_group(&self) -> &ImageGroup {
        self.portrait_group.as_ref().unwrap_or_else(|| ImageGroup::default_instance())
    }
}

impl ::protobuf::Message for Artist {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.gid));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.popularity = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.top_track));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.album_group));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.single_group));
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.compilation_group));
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.appears_on_group));
                },
                9 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.genre));
                },
                10 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.external_id));
                },
                11 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.portrait));
                },
                12 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.biography));
                },
                13 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.activity_period));
                },
                14 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.restriction));
                },
                15 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.related));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_portrait_album_cover = ::std::option::Option::Some(tmp);
                },
                17 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.portrait_group));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.gid {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.popularity {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, *value);
        };
        for value in &self.top_track {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.album_group {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.single_group {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.compilation_group {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.appears_on_group {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.genre {
            my_size += ::protobuf::rt::string_size(9, &value);
        };
        for value in &self.external_id {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.portrait {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.biography {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.activity_period {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.restriction {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.related {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.is_portrait_album_cover.is_some() {
            my_size += 3;
        };
        for value in &self.portrait_group {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.gid.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.popularity {
            try!(os.write_sint32(3, v));
        };
        for v in &self.top_track {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.album_group {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.single_group {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.compilation_group {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.appears_on_group {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.genre {
            try!(os.write_string(9, &v));
        };
        for v in &self.external_id {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.portrait {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.biography {
            try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.activity_period {
            try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.restriction {
            try!(os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.related {
            try!(os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.is_portrait_album_cover {
            try!(os.write_bool(16, v));
        };
        if let Some(v) = self.portrait_group.as_ref() {
            try!(os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Artist>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Artist {
    fn new() -> Artist {
        Artist::new()
    }

    fn descriptor_static(_: ::std::option::Option<Artist>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "gid",
                    Artist::has_gid,
                    Artist::get_gid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Artist::has_name,
                    Artist::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "popularity",
                    Artist::has_popularity,
                    Artist::get_popularity,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "top_track",
                    Artist::get_top_track,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "album_group",
                    Artist::get_album_group,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "single_group",
                    Artist::get_single_group,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "compilation_group",
                    Artist::get_compilation_group,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "appears_on_group",
                    Artist::get_appears_on_group,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "genre",
                    Artist::get_genre,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "external_id",
                    Artist::get_external_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "portrait",
                    Artist::get_portrait,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "biography",
                    Artist::get_biography,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "activity_period",
                    Artist::get_activity_period,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "restriction",
                    Artist::get_restriction,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "related",
                    Artist::get_related,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_portrait_album_cover",
                    Artist::has_is_portrait_album_cover,
                    Artist::get_is_portrait_album_cover,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "portrait_group",
                    Artist::has_portrait_group,
                    Artist::get_portrait_group,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Artist>(
                    "Artist",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Artist {
    fn clear(&mut self) {
        self.clear_gid();
        self.clear_name();
        self.clear_popularity();
        self.clear_top_track();
        self.clear_album_group();
        self.clear_single_group();
        self.clear_compilation_group();
        self.clear_appears_on_group();
        self.clear_genre();
        self.clear_external_id();
        self.clear_portrait();
        self.clear_biography();
        self.clear_activity_period();
        self.clear_restriction();
        self.clear_related();
        self.clear_is_portrait_album_cover();
        self.clear_portrait_group();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Artist {
    fn eq(&self, other: &Artist) -> bool {
        self.gid == other.gid &&
        self.name == other.name &&
        self.popularity == other.popularity &&
        self.top_track == other.top_track &&
        self.album_group == other.album_group &&
        self.single_group == other.single_group &&
        self.compilation_group == other.compilation_group &&
        self.appears_on_group == other.appears_on_group &&
        self.genre == other.genre &&
        self.external_id == other.external_id &&
        self.portrait == other.portrait &&
        self.biography == other.biography &&
        self.activity_period == other.activity_period &&
        self.restriction == other.restriction &&
        self.related == other.related &&
        self.is_portrait_album_cover == other.is_portrait_album_cover &&
        self.portrait_group == other.portrait_group &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Artist {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AlbumGroup {
    // message fields
    album: ::protobuf::RepeatedField<Album>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AlbumGroup {}

impl AlbumGroup {
    pub fn new() -> AlbumGroup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AlbumGroup {
        static mut instance: ::protobuf::lazy::Lazy<AlbumGroup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AlbumGroup,
        };
        unsafe {
            instance.get(|| {
                AlbumGroup {
                    album: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .Album album = 1;

    pub fn clear_album(&mut self) {
        self.album.clear();
    }

    // Param is passed by value, moved
    pub fn set_album(&mut self, v: ::protobuf::RepeatedField<Album>) {
        self.album = v;
    }

    // Mutable pointer to the field.
    pub fn mut_album(&mut self) -> &mut ::protobuf::RepeatedField<Album> {
        &mut self.album
    }

    // Take field
    pub fn take_album(&mut self) -> ::protobuf::RepeatedField<Album> {
        ::std::mem::replace(&mut self.album, ::protobuf::RepeatedField::new())
    }

    pub fn get_album(&self) -> &[Album] {
        &self.album
    }
}

impl ::protobuf::Message for AlbumGroup {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.album));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.album {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.album {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AlbumGroup>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AlbumGroup {
    fn new() -> AlbumGroup {
        AlbumGroup::new()
    }

    fn descriptor_static(_: ::std::option::Option<AlbumGroup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "album",
                    AlbumGroup::get_album,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AlbumGroup>(
                    "AlbumGroup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AlbumGroup {
    fn clear(&mut self) {
        self.clear_album();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AlbumGroup {
    fn eq(&self, other: &AlbumGroup) -> bool {
        self.album == other.album &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AlbumGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Date {
    // message fields
    year: ::std::option::Option<i32>,
    month: ::std::option::Option<i32>,
    day: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Date {}

impl Date {
    pub fn new() -> Date {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Date {
        static mut instance: ::protobuf::lazy::Lazy<Date> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Date,
        };
        unsafe {
            instance.get(|| {
                Date {
                    year: ::std::option::Option::None,
                    month: ::std::option::Option::None,
                    day: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional sint32 year = 1;

    pub fn clear_year(&mut self) {
        self.year = ::std::option::Option::None;
    }

    pub fn has_year(&self) -> bool {
        self.year.is_some()
    }

    // Param is passed by value, moved
    pub fn set_year(&mut self, v: i32) {
        self.year = ::std::option::Option::Some(v);
    }

    pub fn get_year(&self) -> i32 {
        self.year.unwrap_or(0)
    }

    // optional sint32 month = 2;

    pub fn clear_month(&mut self) {
        self.month = ::std::option::Option::None;
    }

    pub fn has_month(&self) -> bool {
        self.month.is_some()
    }

    // Param is passed by value, moved
    pub fn set_month(&mut self, v: i32) {
        self.month = ::std::option::Option::Some(v);
    }

    pub fn get_month(&self) -> i32 {
        self.month.unwrap_or(0)
    }

    // optional sint32 day = 3;

    pub fn clear_day(&mut self) {
        self.day = ::std::option::Option::None;
    }

    pub fn has_day(&self) -> bool {
        self.day.is_some()
    }

    // Param is passed by value, moved
    pub fn set_day(&mut self, v: i32) {
        self.day = ::std::option::Option::Some(v);
    }

    pub fn get_day(&self) -> i32 {
        self.day.unwrap_or(0)
    }
}

impl ::protobuf::Message for Date {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.year = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.month = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.day = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.year {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, *value);
        };
        for value in &self.month {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, *value);
        };
        for value in &self.day {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.year {
            try!(os.write_sint32(1, v));
        };
        if let Some(v) = self.month {
            try!(os.write_sint32(2, v));
        };
        if let Some(v) = self.day {
            try!(os.write_sint32(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Date>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Date {
    fn new() -> Date {
        Date::new()
    }

    fn descriptor_static(_: ::std::option::Option<Date>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "year",
                    Date::has_year,
                    Date::get_year,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "month",
                    Date::has_month,
                    Date::get_month,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "day",
                    Date::has_day,
                    Date::get_day,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Date>(
                    "Date",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Date {
    fn clear(&mut self) {
        self.clear_year();
        self.clear_month();
        self.clear_day();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Date {
    fn eq(&self, other: &Date) -> bool {
        self.year == other.year &&
        self.month == other.month &&
        self.day == other.day &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Date {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Album {
    // message fields
    gid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    name: ::protobuf::SingularField<::std::string::String>,
    artist: ::protobuf::RepeatedField<Artist>,
    typ: ::std::option::Option<Album_Type>,
    label: ::protobuf::SingularField<::std::string::String>,
    date: ::protobuf::SingularPtrField<Date>,
    popularity: ::std::option::Option<i32>,
    genre: ::protobuf::RepeatedField<::std::string::String>,
    cover: ::protobuf::RepeatedField<Image>,
    external_id: ::protobuf::RepeatedField<ExternalId>,
    disc: ::protobuf::RepeatedField<Disc>,
    review: ::protobuf::RepeatedField<::std::string::String>,
    copyright: ::protobuf::RepeatedField<Copyright>,
    restriction: ::protobuf::RepeatedField<Restriction>,
    related: ::protobuf::RepeatedField<Album>,
    sale_period: ::protobuf::RepeatedField<SalePeriod>,
    cover_group: ::protobuf::SingularPtrField<ImageGroup>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Album {}

impl Album {
    pub fn new() -> Album {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Album {
        static mut instance: ::protobuf::lazy::Lazy<Album> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Album,
        };
        unsafe {
            instance.get(|| {
                Album {
                    gid: ::protobuf::SingularField::none(),
                    name: ::protobuf::SingularField::none(),
                    artist: ::protobuf::RepeatedField::new(),
                    typ: ::std::option::Option::None,
                    label: ::protobuf::SingularField::none(),
                    date: ::protobuf::SingularPtrField::none(),
                    popularity: ::std::option::Option::None,
                    genre: ::protobuf::RepeatedField::new(),
                    cover: ::protobuf::RepeatedField::new(),
                    external_id: ::protobuf::RepeatedField::new(),
                    disc: ::protobuf::RepeatedField::new(),
                    review: ::protobuf::RepeatedField::new(),
                    copyright: ::protobuf::RepeatedField::new(),
                    restriction: ::protobuf::RepeatedField::new(),
                    related: ::protobuf::RepeatedField::new(),
                    sale_period: ::protobuf::RepeatedField::new(),
                    cover_group: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes gid = 1;

    pub fn clear_gid(&mut self) {
        self.gid.clear();
    }

    pub fn has_gid(&self) -> bool {
        self.gid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gid(&mut self, v: ::std::vec::Vec<u8>) {
        self.gid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.gid.is_none() {
            self.gid.set_default();
        };
        self.gid.as_mut().unwrap()
    }

    // Take field
    pub fn take_gid(&mut self) -> ::std::vec::Vec<u8> {
        self.gid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_gid(&self) -> &[u8] {
        match self.gid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .Artist artist = 3;

    pub fn clear_artist(&mut self) {
        self.artist.clear();
    }

    // Param is passed by value, moved
    pub fn set_artist(&mut self, v: ::protobuf::RepeatedField<Artist>) {
        self.artist = v;
    }

    // Mutable pointer to the field.
    pub fn mut_artist(&mut self) -> &mut ::protobuf::RepeatedField<Artist> {
        &mut self.artist
    }

    // Take field
    pub fn take_artist(&mut self) -> ::protobuf::RepeatedField<Artist> {
        ::std::mem::replace(&mut self.artist, ::protobuf::RepeatedField::new())
    }

    pub fn get_artist(&self) -> &[Artist] {
        &self.artist
    }

    // optional .Album.Type typ = 4;

    pub fn clear_typ(&mut self) {
        self.typ = ::std::option::Option::None;
    }

    pub fn has_typ(&self) -> bool {
        self.typ.is_some()
    }

    // Param is passed by value, moved
    pub fn set_typ(&mut self, v: Album_Type) {
        self.typ = ::std::option::Option::Some(v);
    }

    pub fn get_typ(&self) -> Album_Type {
        self.typ.unwrap_or(Album_Type::ALBUM)
    }

    // optional string label = 5;

    pub fn clear_label(&mut self) {
        self.label.clear();
    }

    pub fn has_label(&self) -> bool {
        self.label.is_some()
    }

    // Param is passed by value, moved
    pub fn set_label(&mut self, v: ::std::string::String) {
        self.label = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_label(&mut self) -> &mut ::std::string::String {
        if self.label.is_none() {
            self.label.set_default();
        };
        self.label.as_mut().unwrap()
    }

    // Take field
    pub fn take_label(&mut self) -> ::std::string::String {
        self.label.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_label(&self) -> &str {
        match self.label.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .Date date = 6;

    pub fn clear_date(&mut self) {
        self.date.clear();
    }

    pub fn has_date(&self) -> bool {
        self.date.is_some()
    }

    // Param is passed by value, moved
    pub fn set_date(&mut self, v: Date) {
        self.date = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_date(&mut self) -> &mut Date {
        if self.date.is_none() {
            self.date.set_default();
        };
        self.date.as_mut().unwrap()
    }

    // Take field
    pub fn take_date(&mut self) -> Date {
        self.date.take().unwrap_or_else(|| Date::new())
    }

    pub fn get_date(&self) -> &Date {
        self.date.as_ref().unwrap_or_else(|| Date::default_instance())
    }

    // optional sint32 popularity = 7;

    pub fn clear_popularity(&mut self) {
        self.popularity = ::std::option::Option::None;
    }

    pub fn has_popularity(&self) -> bool {
        self.popularity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_popularity(&mut self, v: i32) {
        self.popularity = ::std::option::Option::Some(v);
    }

    pub fn get_popularity(&self) -> i32 {
        self.popularity.unwrap_or(0)
    }

    // repeated string genre = 8;

    pub fn clear_genre(&mut self) {
        self.genre.clear();
    }

    // Param is passed by value, moved
    pub fn set_genre(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.genre = v;
    }

    // Mutable pointer to the field.
    pub fn mut_genre(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.genre
    }

    // Take field
    pub fn take_genre(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.genre, ::protobuf::RepeatedField::new())
    }

    pub fn get_genre(&self) -> &[::std::string::String] {
        &self.genre
    }

    // repeated .Image cover = 9;

    pub fn clear_cover(&mut self) {
        self.cover.clear();
    }

    // Param is passed by value, moved
    pub fn set_cover(&mut self, v: ::protobuf::RepeatedField<Image>) {
        self.cover = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cover(&mut self) -> &mut ::protobuf::RepeatedField<Image> {
        &mut self.cover
    }

    // Take field
    pub fn take_cover(&mut self) -> ::protobuf::RepeatedField<Image> {
        ::std::mem::replace(&mut self.cover, ::protobuf::RepeatedField::new())
    }

    pub fn get_cover(&self) -> &[Image] {
        &self.cover
    }

    // repeated .ExternalId external_id = 10;

    pub fn clear_external_id(&mut self) {
        self.external_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_external_id(&mut self, v: ::protobuf::RepeatedField<ExternalId>) {
        self.external_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_external_id(&mut self) -> &mut ::protobuf::RepeatedField<ExternalId> {
        &mut self.external_id
    }

    // Take field
    pub fn take_external_id(&mut self) -> ::protobuf::RepeatedField<ExternalId> {
        ::std::mem::replace(&mut self.external_id, ::protobuf::RepeatedField::new())
    }

    pub fn get_external_id(&self) -> &[ExternalId] {
        &self.external_id
    }

    // repeated .Disc disc = 11;

    pub fn clear_disc(&mut self) {
        self.disc.clear();
    }

    // Param is passed by value, moved
    pub fn set_disc(&mut self, v: ::protobuf::RepeatedField<Disc>) {
        self.disc = v;
    }

    // Mutable pointer to the field.
    pub fn mut_disc(&mut self) -> &mut ::protobuf::RepeatedField<Disc> {
        &mut self.disc
    }

    // Take field
    pub fn take_disc(&mut self) -> ::protobuf::RepeatedField<Disc> {
        ::std::mem::replace(&mut self.disc, ::protobuf::RepeatedField::new())
    }

    pub fn get_disc(&self) -> &[Disc] {
        &self.disc
    }

    // repeated string review = 12;

    pub fn clear_review(&mut self) {
        self.review.clear();
    }

    // Param is passed by value, moved
    pub fn set_review(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.review = v;
    }

    // Mutable pointer to the field.
    pub fn mut_review(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.review
    }

    // Take field
    pub fn take_review(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.review, ::protobuf::RepeatedField::new())
    }

    pub fn get_review(&self) -> &[::std::string::String] {
        &self.review
    }

    // repeated .Copyright copyright = 13;

    pub fn clear_copyright(&mut self) {
        self.copyright.clear();
    }

    // Param is passed by value, moved
    pub fn set_copyright(&mut self, v: ::protobuf::RepeatedField<Copyright>) {
        self.copyright = v;
    }

    // Mutable pointer to the field.
    pub fn mut_copyright(&mut self) -> &mut ::protobuf::RepeatedField<Copyright> {
        &mut self.copyright
    }

    // Take field
    pub fn take_copyright(&mut self) -> ::protobuf::RepeatedField<Copyright> {
        ::std::mem::replace(&mut self.copyright, ::protobuf::RepeatedField::new())
    }

    pub fn get_copyright(&self) -> &[Copyright] {
        &self.copyright
    }

    // repeated .Restriction restriction = 14;

    pub fn clear_restriction(&mut self) {
        self.restriction.clear();
    }

    // Param is passed by value, moved
    pub fn set_restriction(&mut self, v: ::protobuf::RepeatedField<Restriction>) {
        self.restriction = v;
    }

    // Mutable pointer to the field.
    pub fn mut_restriction(&mut self) -> &mut ::protobuf::RepeatedField<Restriction> {
        &mut self.restriction
    }

    // Take field
    pub fn take_restriction(&mut self) -> ::protobuf::RepeatedField<Restriction> {
        ::std::mem::replace(&mut self.restriction, ::protobuf::RepeatedField::new())
    }

    pub fn get_restriction(&self) -> &[Restriction] {
        &self.restriction
    }

    // repeated .Album related = 15;

    pub fn clear_related(&mut self) {
        self.related.clear();
    }

    // Param is passed by value, moved
    pub fn set_related(&mut self, v: ::protobuf::RepeatedField<Album>) {
        self.related = v;
    }

    // Mutable pointer to the field.
    pub fn mut_related(&mut self) -> &mut ::protobuf::RepeatedField<Album> {
        &mut self.related
    }

    // Take field
    pub fn take_related(&mut self) -> ::protobuf::RepeatedField<Album> {
        ::std::mem::replace(&mut self.related, ::protobuf::RepeatedField::new())
    }

    pub fn get_related(&self) -> &[Album] {
        &self.related
    }

    // repeated .SalePeriod sale_period = 16;

    pub fn clear_sale_period(&mut self) {
        self.sale_period.clear();
    }

    // Param is passed by value, moved
    pub fn set_sale_period(&mut self, v: ::protobuf::RepeatedField<SalePeriod>) {
        self.sale_period = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sale_period(&mut self) -> &mut ::protobuf::RepeatedField<SalePeriod> {
        &mut self.sale_period
    }

    // Take field
    pub fn take_sale_period(&mut self) -> ::protobuf::RepeatedField<SalePeriod> {
        ::std::mem::replace(&mut self.sale_period, ::protobuf::RepeatedField::new())
    }

    pub fn get_sale_period(&self) -> &[SalePeriod] {
        &self.sale_period
    }

    // optional .ImageGroup cover_group = 17;

    pub fn clear_cover_group(&mut self) {
        self.cover_group.clear();
    }

    pub fn has_cover_group(&self) -> bool {
        self.cover_group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cover_group(&mut self, v: ImageGroup) {
        self.cover_group = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cover_group(&mut self) -> &mut ImageGroup {
        if self.cover_group.is_none() {
            self.cover_group.set_default();
        };
        self.cover_group.as_mut().unwrap()
    }

    // Take field
    pub fn take_cover_group(&mut self) -> ImageGroup {
        self.cover_group.take().unwrap_or_else(|| ImageGroup::new())
    }

    pub fn get_cover_group(&self) -> &ImageGroup {
        self.cover_group.as_ref().unwrap_or_else(|| ImageGroup::default_instance())
    }
}

impl ::protobuf::Message for Album {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.gid));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.artist));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.typ = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.label));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.date));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.popularity = ::std::option::Option::Some(tmp);
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.genre));
                },
                9 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cover));
                },
                10 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.external_id));
                },
                11 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.disc));
                },
                12 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.review));
                },
                13 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.copyright));
                },
                14 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.restriction));
                },
                15 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.related));
                },
                16 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.sale_period));
                },
                17 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cover_group));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.gid {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.artist {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.typ {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        for value in &self.label {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in &self.date {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.popularity {
            my_size += ::protobuf::rt::value_varint_zigzag_size(7, *value);
        };
        for value in &self.genre {
            my_size += ::protobuf::rt::string_size(8, &value);
        };
        for value in &self.cover {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.external_id {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.disc {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.review {
            my_size += ::protobuf::rt::string_size(12, &value);
        };
        for value in &self.copyright {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.restriction {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.related {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.sale_period {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.cover_group {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.gid.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(2, &v));
        };
        for v in &self.artist {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.typ {
            try!(os.write_enum(4, v.value()));
        };
        if let Some(v) = self.label.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.date.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.popularity {
            try!(os.write_sint32(7, v));
        };
        for v in &self.genre {
            try!(os.write_string(8, &v));
        };
        for v in &self.cover {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.external_id {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.disc {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.review {
            try!(os.write_string(12, &v));
        };
        for v in &self.copyright {
            try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.restriction {
            try!(os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.related {
            try!(os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.sale_period {
            try!(os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cover_group.as_ref() {
            try!(os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Album>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Album {
    fn new() -> Album {
        Album::new()
    }

    fn descriptor_static(_: ::std::option::Option<Album>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "gid",
                    Album::has_gid,
                    Album::get_gid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Album::has_name,
                    Album::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "artist",
                    Album::get_artist,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "typ",
                    Album::has_typ,
                    Album::get_typ,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "label",
                    Album::has_label,
                    Album::get_label,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "date",
                    Album::has_date,
                    Album::get_date,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "popularity",
                    Album::has_popularity,
                    Album::get_popularity,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "genre",
                    Album::get_genre,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "cover",
                    Album::get_cover,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "external_id",
                    Album::get_external_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "disc",
                    Album::get_disc,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "review",
                    Album::get_review,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "copyright",
                    Album::get_copyright,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "restriction",
                    Album::get_restriction,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "related",
                    Album::get_related,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "sale_period",
                    Album::get_sale_period,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cover_group",
                    Album::has_cover_group,
                    Album::get_cover_group,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Album>(
                    "Album",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Album {
    fn clear(&mut self) {
        self.clear_gid();
        self.clear_name();
        self.clear_artist();
        self.clear_typ();
        self.clear_label();
        self.clear_date();
        self.clear_popularity();
        self.clear_genre();
        self.clear_cover();
        self.clear_external_id();
        self.clear_disc();
        self.clear_review();
        self.clear_copyright();
        self.clear_restriction();
        self.clear_related();
        self.clear_sale_period();
        self.clear_cover_group();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Album {
    fn eq(&self, other: &Album) -> bool {
        self.gid == other.gid &&
        self.name == other.name &&
        self.artist == other.artist &&
        self.typ == other.typ &&
        self.label == other.label &&
        self.date == other.date &&
        self.popularity == other.popularity &&
        self.genre == other.genre &&
        self.cover == other.cover &&
        self.external_id == other.external_id &&
        self.disc == other.disc &&
        self.review == other.review &&
        self.copyright == other.copyright &&
        self.restriction == other.restriction &&
        self.related == other.related &&
        self.sale_period == other.sale_period &&
        self.cover_group == other.cover_group &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Album {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Album_Type {
    ALBUM = 1,
    SINGLE = 2,
    COMPILATION = 3,
}

impl ::protobuf::ProtobufEnum for Album_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Album_Type> {
        match value {
            1 => ::std::option::Option::Some(Album_Type::ALBUM),
            2 => ::std::option::Option::Some(Album_Type::SINGLE),
            3 => ::std::option::Option::Some(Album_Type::COMPILATION),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Album_Type] = &[
            Album_Type::ALBUM,
            Album_Type::SINGLE,
            Album_Type::COMPILATION,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Album_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Album_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Album_Type {
}

#[derive(Clone,Default)]
pub struct Track {
    // message fields
    gid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    name: ::protobuf::SingularField<::std::string::String>,
    album: ::protobuf::SingularPtrField<Album>,
    artist: ::protobuf::RepeatedField<Artist>,
    number: ::std::option::Option<i32>,
    disc_number: ::std::option::Option<i32>,
    duration: ::std::option::Option<i32>,
    popularity: ::std::option::Option<i32>,
    explicit: ::std::option::Option<bool>,
    external_id: ::protobuf::RepeatedField<ExternalId>,
    restriction: ::protobuf::RepeatedField<Restriction>,
    file: ::protobuf::RepeatedField<AudioFile>,
    alternative: ::protobuf::RepeatedField<Track>,
    sale_period: ::protobuf::RepeatedField<SalePeriod>,
    preview: ::protobuf::RepeatedField<AudioFile>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Track {}

impl Track {
    pub fn new() -> Track {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Track {
        static mut instance: ::protobuf::lazy::Lazy<Track> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Track,
        };
        unsafe {
            instance.get(|| {
                Track {
                    gid: ::protobuf::SingularField::none(),
                    name: ::protobuf::SingularField::none(),
                    album: ::protobuf::SingularPtrField::none(),
                    artist: ::protobuf::RepeatedField::new(),
                    number: ::std::option::Option::None,
                    disc_number: ::std::option::Option::None,
                    duration: ::std::option::Option::None,
                    popularity: ::std::option::Option::None,
                    explicit: ::std::option::Option::None,
                    external_id: ::protobuf::RepeatedField::new(),
                    restriction: ::protobuf::RepeatedField::new(),
                    file: ::protobuf::RepeatedField::new(),
                    alternative: ::protobuf::RepeatedField::new(),
                    sale_period: ::protobuf::RepeatedField::new(),
                    preview: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes gid = 1;

    pub fn clear_gid(&mut self) {
        self.gid.clear();
    }

    pub fn has_gid(&self) -> bool {
        self.gid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gid(&mut self, v: ::std::vec::Vec<u8>) {
        self.gid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.gid.is_none() {
            self.gid.set_default();
        };
        self.gid.as_mut().unwrap()
    }

    // Take field
    pub fn take_gid(&mut self) -> ::std::vec::Vec<u8> {
        self.gid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_gid(&self) -> &[u8] {
        match self.gid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .Album album = 3;

    pub fn clear_album(&mut self) {
        self.album.clear();
    }

    pub fn has_album(&self) -> bool {
        self.album.is_some()
    }

    // Param is passed by value, moved
    pub fn set_album(&mut self, v: Album) {
        self.album = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_album(&mut self) -> &mut Album {
        if self.album.is_none() {
            self.album.set_default();
        };
        self.album.as_mut().unwrap()
    }

    // Take field
    pub fn take_album(&mut self) -> Album {
        self.album.take().unwrap_or_else(|| Album::new())
    }

    pub fn get_album(&self) -> &Album {
        self.album.as_ref().unwrap_or_else(|| Album::default_instance())
    }

    // repeated .Artist artist = 4;

    pub fn clear_artist(&mut self) {
        self.artist.clear();
    }

    // Param is passed by value, moved
    pub fn set_artist(&mut self, v: ::protobuf::RepeatedField<Artist>) {
        self.artist = v;
    }

    // Mutable pointer to the field.
    pub fn mut_artist(&mut self) -> &mut ::protobuf::RepeatedField<Artist> {
        &mut self.artist
    }

    // Take field
    pub fn take_artist(&mut self) -> ::protobuf::RepeatedField<Artist> {
        ::std::mem::replace(&mut self.artist, ::protobuf::RepeatedField::new())
    }

    pub fn get_artist(&self) -> &[Artist] {
        &self.artist
    }

    // optional sint32 number = 5;

    pub fn clear_number(&mut self) {
        self.number = ::std::option::Option::None;
    }

    pub fn has_number(&self) -> bool {
        self.number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: i32) {
        self.number = ::std::option::Option::Some(v);
    }

    pub fn get_number(&self) -> i32 {
        self.number.unwrap_or(0)
    }

    // optional sint32 disc_number = 6;

    pub fn clear_disc_number(&mut self) {
        self.disc_number = ::std::option::Option::None;
    }

    pub fn has_disc_number(&self) -> bool {
        self.disc_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_disc_number(&mut self, v: i32) {
        self.disc_number = ::std::option::Option::Some(v);
    }

    pub fn get_disc_number(&self) -> i32 {
        self.disc_number.unwrap_or(0)
    }

    // optional sint32 duration = 7;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: i32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> i32 {
        self.duration.unwrap_or(0)
    }

    // optional sint32 popularity = 8;

    pub fn clear_popularity(&mut self) {
        self.popularity = ::std::option::Option::None;
    }

    pub fn has_popularity(&self) -> bool {
        self.popularity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_popularity(&mut self, v: i32) {
        self.popularity = ::std::option::Option::Some(v);
    }

    pub fn get_popularity(&self) -> i32 {
        self.popularity.unwrap_or(0)
    }

    // optional bool explicit = 9;

    pub fn clear_explicit(&mut self) {
        self.explicit = ::std::option::Option::None;
    }

    pub fn has_explicit(&self) -> bool {
        self.explicit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_explicit(&mut self, v: bool) {
        self.explicit = ::std::option::Option::Some(v);
    }

    pub fn get_explicit(&self) -> bool {
        self.explicit.unwrap_or(false)
    }

    // repeated .ExternalId external_id = 10;

    pub fn clear_external_id(&mut self) {
        self.external_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_external_id(&mut self, v: ::protobuf::RepeatedField<ExternalId>) {
        self.external_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_external_id(&mut self) -> &mut ::protobuf::RepeatedField<ExternalId> {
        &mut self.external_id
    }

    // Take field
    pub fn take_external_id(&mut self) -> ::protobuf::RepeatedField<ExternalId> {
        ::std::mem::replace(&mut self.external_id, ::protobuf::RepeatedField::new())
    }

    pub fn get_external_id(&self) -> &[ExternalId] {
        &self.external_id
    }

    // repeated .Restriction restriction = 11;

    pub fn clear_restriction(&mut self) {
        self.restriction.clear();
    }

    // Param is passed by value, moved
    pub fn set_restriction(&mut self, v: ::protobuf::RepeatedField<Restriction>) {
        self.restriction = v;
    }

    // Mutable pointer to the field.
    pub fn mut_restriction(&mut self) -> &mut ::protobuf::RepeatedField<Restriction> {
        &mut self.restriction
    }

    // Take field
    pub fn take_restriction(&mut self) -> ::protobuf::RepeatedField<Restriction> {
        ::std::mem::replace(&mut self.restriction, ::protobuf::RepeatedField::new())
    }

    pub fn get_restriction(&self) -> &[Restriction] {
        &self.restriction
    }

    // repeated .AudioFile file = 12;

    pub fn clear_file(&mut self) {
        self.file.clear();
    }

    // Param is passed by value, moved
    pub fn set_file(&mut self, v: ::protobuf::RepeatedField<AudioFile>) {
        self.file = v;
    }

    // Mutable pointer to the field.
    pub fn mut_file(&mut self) -> &mut ::protobuf::RepeatedField<AudioFile> {
        &mut self.file
    }

    // Take field
    pub fn take_file(&mut self) -> ::protobuf::RepeatedField<AudioFile> {
        ::std::mem::replace(&mut self.file, ::protobuf::RepeatedField::new())
    }

    pub fn get_file(&self) -> &[AudioFile] {
        &self.file
    }

    // repeated .Track alternative = 13;

    pub fn clear_alternative(&mut self) {
        self.alternative.clear();
    }

    // Param is passed by value, moved
    pub fn set_alternative(&mut self, v: ::protobuf::RepeatedField<Track>) {
        self.alternative = v;
    }

    // Mutable pointer to the field.
    pub fn mut_alternative(&mut self) -> &mut ::protobuf::RepeatedField<Track> {
        &mut self.alternative
    }

    // Take field
    pub fn take_alternative(&mut self) -> ::protobuf::RepeatedField<Track> {
        ::std::mem::replace(&mut self.alternative, ::protobuf::RepeatedField::new())
    }

    pub fn get_alternative(&self) -> &[Track] {
        &self.alternative
    }

    // repeated .SalePeriod sale_period = 14;

    pub fn clear_sale_period(&mut self) {
        self.sale_period.clear();
    }

    // Param is passed by value, moved
    pub fn set_sale_period(&mut self, v: ::protobuf::RepeatedField<SalePeriod>) {
        self.sale_period = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sale_period(&mut self) -> &mut ::protobuf::RepeatedField<SalePeriod> {
        &mut self.sale_period
    }

    // Take field
    pub fn take_sale_period(&mut self) -> ::protobuf::RepeatedField<SalePeriod> {
        ::std::mem::replace(&mut self.sale_period, ::protobuf::RepeatedField::new())
    }

    pub fn get_sale_period(&self) -> &[SalePeriod] {
        &self.sale_period
    }

    // repeated .AudioFile preview = 15;

    pub fn clear_preview(&mut self) {
        self.preview.clear();
    }

    // Param is passed by value, moved
    pub fn set_preview(&mut self, v: ::protobuf::RepeatedField<AudioFile>) {
        self.preview = v;
    }

    // Mutable pointer to the field.
    pub fn mut_preview(&mut self) -> &mut ::protobuf::RepeatedField<AudioFile> {
        &mut self.preview
    }

    // Take field
    pub fn take_preview(&mut self) -> ::protobuf::RepeatedField<AudioFile> {
        ::std::mem::replace(&mut self.preview, ::protobuf::RepeatedField::new())
    }

    pub fn get_preview(&self) -> &[AudioFile] {
        &self.preview
    }
}

impl ::protobuf::Message for Track {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.gid));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.album));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.artist));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.number = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.disc_number = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.duration = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.popularity = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.explicit = ::std::option::Option::Some(tmp);
                },
                10 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.external_id));
                },
                11 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.restriction));
                },
                12 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.file));
                },
                13 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.alternative));
                },
                14 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.sale_period));
                },
                15 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.preview));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.gid {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.album {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.artist {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.number {
            my_size += ::protobuf::rt::value_varint_zigzag_size(5, *value);
        };
        for value in &self.disc_number {
            my_size += ::protobuf::rt::value_varint_zigzag_size(6, *value);
        };
        for value in &self.duration {
            my_size += ::protobuf::rt::value_varint_zigzag_size(7, *value);
        };
        for value in &self.popularity {
            my_size += ::protobuf::rt::value_varint_zigzag_size(8, *value);
        };
        if self.explicit.is_some() {
            my_size += 2;
        };
        for value in &self.external_id {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.restriction {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.file {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.alternative {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.sale_period {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.preview {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.gid.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.album.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.artist {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.number {
            try!(os.write_sint32(5, v));
        };
        if let Some(v) = self.disc_number {
            try!(os.write_sint32(6, v));
        };
        if let Some(v) = self.duration {
            try!(os.write_sint32(7, v));
        };
        if let Some(v) = self.popularity {
            try!(os.write_sint32(8, v));
        };
        if let Some(v) = self.explicit {
            try!(os.write_bool(9, v));
        };
        for v in &self.external_id {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.restriction {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.file {
            try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.alternative {
            try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.sale_period {
            try!(os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.preview {
            try!(os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Track>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Track {
    fn new() -> Track {
        Track::new()
    }

    fn descriptor_static(_: ::std::option::Option<Track>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "gid",
                    Track::has_gid,
                    Track::get_gid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Track::has_name,
                    Track::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "album",
                    Track::has_album,
                    Track::get_album,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "artist",
                    Track::get_artist,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "number",
                    Track::has_number,
                    Track::get_number,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "disc_number",
                    Track::has_disc_number,
                    Track::get_disc_number,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "duration",
                    Track::has_duration,
                    Track::get_duration,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "popularity",
                    Track::has_popularity,
                    Track::get_popularity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "explicit",
                    Track::has_explicit,
                    Track::get_explicit,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "external_id",
                    Track::get_external_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "restriction",
                    Track::get_restriction,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "file",
                    Track::get_file,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "alternative",
                    Track::get_alternative,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "sale_period",
                    Track::get_sale_period,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "preview",
                    Track::get_preview,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Track>(
                    "Track",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Track {
    fn clear(&mut self) {
        self.clear_gid();
        self.clear_name();
        self.clear_album();
        self.clear_artist();
        self.clear_number();
        self.clear_disc_number();
        self.clear_duration();
        self.clear_popularity();
        self.clear_explicit();
        self.clear_external_id();
        self.clear_restriction();
        self.clear_file();
        self.clear_alternative();
        self.clear_sale_period();
        self.clear_preview();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Track {
    fn eq(&self, other: &Track) -> bool {
        self.gid == other.gid &&
        self.name == other.name &&
        self.album == other.album &&
        self.artist == other.artist &&
        self.number == other.number &&
        self.disc_number == other.disc_number &&
        self.duration == other.duration &&
        self.popularity == other.popularity &&
        self.explicit == other.explicit &&
        self.external_id == other.external_id &&
        self.restriction == other.restriction &&
        self.file == other.file &&
        self.alternative == other.alternative &&
        self.sale_period == other.sale_period &&
        self.preview == other.preview &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Track {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Image {
    // message fields
    file_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    size: ::std::option::Option<Image_Size>,
    width: ::std::option::Option<i32>,
    height: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Image {}

impl Image {
    pub fn new() -> Image {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Image {
        static mut instance: ::protobuf::lazy::Lazy<Image> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Image,
        };
        unsafe {
            instance.get(|| {
                Image {
                    file_id: ::protobuf::SingularField::none(),
                    size: ::std::option::Option::None,
                    width: ::std::option::Option::None,
                    height: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes file_id = 1;

    pub fn clear_file_id(&mut self) {
        self.file_id.clear();
    }

    pub fn has_file_id(&self) -> bool {
        self.file_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.file_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.file_id.is_none() {
            self.file_id.set_default();
        };
        self.file_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_file_id(&mut self) -> ::std::vec::Vec<u8> {
        self.file_id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_file_id(&self) -> &[u8] {
        match self.file_id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .Image.Size size = 2;

    pub fn clear_size(&mut self) {
        self.size = ::std::option::Option::None;
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: Image_Size) {
        self.size = ::std::option::Option::Some(v);
    }

    pub fn get_size(&self) -> Image_Size {
        self.size.unwrap_or(Image_Size::DEFAULT)
    }

    // optional sint32 width = 3;

    pub fn clear_width(&mut self) {
        self.width = ::std::option::Option::None;
    }

    pub fn has_width(&self) -> bool {
        self.width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: i32) {
        self.width = ::std::option::Option::Some(v);
    }

    pub fn get_width(&self) -> i32 {
        self.width.unwrap_or(0)
    }

    // optional sint32 height = 4;

    pub fn clear_height(&mut self) {
        self.height = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: i32) {
        self.height = ::std::option::Option::Some(v);
    }

    pub fn get_height(&self) -> i32 {
        self.height.unwrap_or(0)
    }
}

impl ::protobuf::Message for Image {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.file_id));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.size = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.width = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.height = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.file_id {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.size {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.width {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, *value);
        };
        for value in &self.height {
            my_size += ::protobuf::rt::value_varint_zigzag_size(4, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.file_id.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.size {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.width {
            try!(os.write_sint32(3, v));
        };
        if let Some(v) = self.height {
            try!(os.write_sint32(4, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Image>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Image {
    fn new() -> Image {
        Image::new()
    }

    fn descriptor_static(_: ::std::option::Option<Image>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "file_id",
                    Image::has_file_id,
                    Image::get_file_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "size",
                    Image::has_size,
                    Image::get_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "width",
                    Image::has_width,
                    Image::get_width,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "height",
                    Image::has_height,
                    Image::get_height,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Image>(
                    "Image",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Image {
    fn clear(&mut self) {
        self.clear_file_id();
        self.clear_size();
        self.clear_width();
        self.clear_height();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Image {
    fn eq(&self, other: &Image) -> bool {
        self.file_id == other.file_id &&
        self.size == other.size &&
        self.width == other.width &&
        self.height == other.height &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Image {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Image_Size {
    DEFAULT = 0,
    SMALL = 1,
    LARGE = 2,
    XLARGE = 3,
}

impl ::protobuf::ProtobufEnum for Image_Size {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Image_Size> {
        match value {
            0 => ::std::option::Option::Some(Image_Size::DEFAULT),
            1 => ::std::option::Option::Some(Image_Size::SMALL),
            2 => ::std::option::Option::Some(Image_Size::LARGE),
            3 => ::std::option::Option::Some(Image_Size::XLARGE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Image_Size] = &[
            Image_Size::DEFAULT,
            Image_Size::SMALL,
            Image_Size::LARGE,
            Image_Size::XLARGE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Image_Size>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Image_Size", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Image_Size {
}

#[derive(Clone,Default)]
pub struct ImageGroup {
    // message fields
    image: ::protobuf::RepeatedField<Image>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ImageGroup {}

impl ImageGroup {
    pub fn new() -> ImageGroup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ImageGroup {
        static mut instance: ::protobuf::lazy::Lazy<ImageGroup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ImageGroup,
        };
        unsafe {
            instance.get(|| {
                ImageGroup {
                    image: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .Image image = 1;

    pub fn clear_image(&mut self) {
        self.image.clear();
    }

    // Param is passed by value, moved
    pub fn set_image(&mut self, v: ::protobuf::RepeatedField<Image>) {
        self.image = v;
    }

    // Mutable pointer to the field.
    pub fn mut_image(&mut self) -> &mut ::protobuf::RepeatedField<Image> {
        &mut self.image
    }

    // Take field
    pub fn take_image(&mut self) -> ::protobuf::RepeatedField<Image> {
        ::std::mem::replace(&mut self.image, ::protobuf::RepeatedField::new())
    }

    pub fn get_image(&self) -> &[Image] {
        &self.image
    }
}

impl ::protobuf::Message for ImageGroup {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.image));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.image {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.image {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ImageGroup>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ImageGroup {
    fn new() -> ImageGroup {
        ImageGroup::new()
    }

    fn descriptor_static(_: ::std::option::Option<ImageGroup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "image",
                    ImageGroup::get_image,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ImageGroup>(
                    "ImageGroup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ImageGroup {
    fn clear(&mut self) {
        self.clear_image();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ImageGroup {
    fn eq(&self, other: &ImageGroup) -> bool {
        self.image == other.image &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ImageGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Biography {
    // message fields
    text: ::protobuf::SingularField<::std::string::String>,
    portrait: ::protobuf::RepeatedField<Image>,
    portrait_group: ::protobuf::RepeatedField<ImageGroup>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Biography {}

impl Biography {
    pub fn new() -> Biography {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Biography {
        static mut instance: ::protobuf::lazy::Lazy<Biography> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Biography,
        };
        unsafe {
            instance.get(|| {
                Biography {
                    text: ::protobuf::SingularField::none(),
                    portrait: ::protobuf::RepeatedField::new(),
                    portrait_group: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string text = 1;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.text = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        if self.text.is_none() {
            self.text.set_default();
        };
        self.text.as_mut().unwrap()
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        self.text.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_text(&self) -> &str {
        match self.text.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .Image portrait = 2;

    pub fn clear_portrait(&mut self) {
        self.portrait.clear();
    }

    // Param is passed by value, moved
    pub fn set_portrait(&mut self, v: ::protobuf::RepeatedField<Image>) {
        self.portrait = v;
    }

    // Mutable pointer to the field.
    pub fn mut_portrait(&mut self) -> &mut ::protobuf::RepeatedField<Image> {
        &mut self.portrait
    }

    // Take field
    pub fn take_portrait(&mut self) -> ::protobuf::RepeatedField<Image> {
        ::std::mem::replace(&mut self.portrait, ::protobuf::RepeatedField::new())
    }

    pub fn get_portrait(&self) -> &[Image] {
        &self.portrait
    }

    // repeated .ImageGroup portrait_group = 3;

    pub fn clear_portrait_group(&mut self) {
        self.portrait_group.clear();
    }

    // Param is passed by value, moved
    pub fn set_portrait_group(&mut self, v: ::protobuf::RepeatedField<ImageGroup>) {
        self.portrait_group = v;
    }

    // Mutable pointer to the field.
    pub fn mut_portrait_group(&mut self) -> &mut ::protobuf::RepeatedField<ImageGroup> {
        &mut self.portrait_group
    }

    // Take field
    pub fn take_portrait_group(&mut self) -> ::protobuf::RepeatedField<ImageGroup> {
        ::std::mem::replace(&mut self.portrait_group, ::protobuf::RepeatedField::new())
    }

    pub fn get_portrait_group(&self) -> &[ImageGroup] {
        &self.portrait_group
    }
}

impl ::protobuf::Message for Biography {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.portrait));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.portrait_group));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.text {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.portrait {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.portrait_group {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.text.as_ref() {
            try!(os.write_string(1, &v));
        };
        for v in &self.portrait {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.portrait_group {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Biography>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Biography {
    fn new() -> Biography {
        Biography::new()
    }

    fn descriptor_static(_: ::std::option::Option<Biography>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "text",
                    Biography::has_text,
                    Biography::get_text,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "portrait",
                    Biography::get_portrait,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "portrait_group",
                    Biography::get_portrait_group,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Biography>(
                    "Biography",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Biography {
    fn clear(&mut self) {
        self.clear_text();
        self.clear_portrait();
        self.clear_portrait_group();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Biography {
    fn eq(&self, other: &Biography) -> bool {
        self.text == other.text &&
        self.portrait == other.portrait &&
        self.portrait_group == other.portrait_group &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Biography {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Disc {
    // message fields
    number: ::std::option::Option<i32>,
    name: ::protobuf::SingularField<::std::string::String>,
    track: ::protobuf::RepeatedField<Track>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Disc {}

impl Disc {
    pub fn new() -> Disc {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Disc {
        static mut instance: ::protobuf::lazy::Lazy<Disc> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Disc,
        };
        unsafe {
            instance.get(|| {
                Disc {
                    number: ::std::option::Option::None,
                    name: ::protobuf::SingularField::none(),
                    track: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional sint32 number = 1;

    pub fn clear_number(&mut self) {
        self.number = ::std::option::Option::None;
    }

    pub fn has_number(&self) -> bool {
        self.number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: i32) {
        self.number = ::std::option::Option::Some(v);
    }

    pub fn get_number(&self) -> i32 {
        self.number.unwrap_or(0)
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .Track track = 3;

    pub fn clear_track(&mut self) {
        self.track.clear();
    }

    // Param is passed by value, moved
    pub fn set_track(&mut self, v: ::protobuf::RepeatedField<Track>) {
        self.track = v;
    }

    // Mutable pointer to the field.
    pub fn mut_track(&mut self) -> &mut ::protobuf::RepeatedField<Track> {
        &mut self.track
    }

    // Take field
    pub fn take_track(&mut self) -> ::protobuf::RepeatedField<Track> {
        ::std::mem::replace(&mut self.track, ::protobuf::RepeatedField::new())
    }

    pub fn get_track(&self) -> &[Track] {
        &self.track
    }
}

impl ::protobuf::Message for Disc {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.number = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.track));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.number {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, *value);
        };
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.track {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.number {
            try!(os.write_sint32(1, v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(2, &v));
        };
        for v in &self.track {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Disc>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Disc {
    fn new() -> Disc {
        Disc::new()
    }

    fn descriptor_static(_: ::std::option::Option<Disc>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "number",
                    Disc::has_number,
                    Disc::get_number,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Disc::has_name,
                    Disc::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "track",
                    Disc::get_track,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Disc>(
                    "Disc",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Disc {
    fn clear(&mut self) {
        self.clear_number();
        self.clear_name();
        self.clear_track();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Disc {
    fn eq(&self, other: &Disc) -> bool {
        self.number == other.number &&
        self.name == other.name &&
        self.track == other.track &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Disc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Copyright {
    // message fields
    typ: ::std::option::Option<Copyright_Type>,
    text: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Copyright {}

impl Copyright {
    pub fn new() -> Copyright {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Copyright {
        static mut instance: ::protobuf::lazy::Lazy<Copyright> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Copyright,
        };
        unsafe {
            instance.get(|| {
                Copyright {
                    typ: ::std::option::Option::None,
                    text: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .Copyright.Type typ = 1;

    pub fn clear_typ(&mut self) {
        self.typ = ::std::option::Option::None;
    }

    pub fn has_typ(&self) -> bool {
        self.typ.is_some()
    }

    // Param is passed by value, moved
    pub fn set_typ(&mut self, v: Copyright_Type) {
        self.typ = ::std::option::Option::Some(v);
    }

    pub fn get_typ(&self) -> Copyright_Type {
        self.typ.unwrap_or(Copyright_Type::P)
    }

    // optional string text = 2;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.text = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        if self.text.is_none() {
            self.text.set_default();
        };
        self.text.as_mut().unwrap()
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        self.text.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_text(&self) -> &str {
        match self.text.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Copyright {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.typ = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.typ {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.text {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.typ {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.text.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Copyright>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Copyright {
    fn new() -> Copyright {
        Copyright::new()
    }

    fn descriptor_static(_: ::std::option::Option<Copyright>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "typ",
                    Copyright::has_typ,
                    Copyright::get_typ,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "text",
                    Copyright::has_text,
                    Copyright::get_text,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Copyright>(
                    "Copyright",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Copyright {
    fn clear(&mut self) {
        self.clear_typ();
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Copyright {
    fn eq(&self, other: &Copyright) -> bool {
        self.typ == other.typ &&
        self.text == other.text &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Copyright {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Copyright_Type {
    P = 0,
    C = 1,
}

impl ::protobuf::ProtobufEnum for Copyright_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Copyright_Type> {
        match value {
            0 => ::std::option::Option::Some(Copyright_Type::P),
            1 => ::std::option::Option::Some(Copyright_Type::C),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Copyright_Type] = &[
            Copyright_Type::P,
            Copyright_Type::C,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Copyright_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Copyright_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Copyright_Type {
}

#[derive(Clone,Default)]
pub struct Restriction {
    // message fields
    countries_allowed: ::protobuf::SingularField<::std::string::String>,
    countries_forbidden: ::protobuf::SingularField<::std::string::String>,
    typ: ::std::option::Option<Restriction_Type>,
    catalogue_str: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Restriction {}

impl Restriction {
    pub fn new() -> Restriction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Restriction {
        static mut instance: ::protobuf::lazy::Lazy<Restriction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Restriction,
        };
        unsafe {
            instance.get(|| {
                Restriction {
                    countries_allowed: ::protobuf::SingularField::none(),
                    countries_forbidden: ::protobuf::SingularField::none(),
                    typ: ::std::option::Option::None,
                    catalogue_str: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string countries_allowed = 2;

    pub fn clear_countries_allowed(&mut self) {
        self.countries_allowed.clear();
    }

    pub fn has_countries_allowed(&self) -> bool {
        self.countries_allowed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_countries_allowed(&mut self, v: ::std::string::String) {
        self.countries_allowed = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_countries_allowed(&mut self) -> &mut ::std::string::String {
        if self.countries_allowed.is_none() {
            self.countries_allowed.set_default();
        };
        self.countries_allowed.as_mut().unwrap()
    }

    // Take field
    pub fn take_countries_allowed(&mut self) -> ::std::string::String {
        self.countries_allowed.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_countries_allowed(&self) -> &str {
        match self.countries_allowed.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string countries_forbidden = 3;

    pub fn clear_countries_forbidden(&mut self) {
        self.countries_forbidden.clear();
    }

    pub fn has_countries_forbidden(&self) -> bool {
        self.countries_forbidden.is_some()
    }

    // Param is passed by value, moved
    pub fn set_countries_forbidden(&mut self, v: ::std::string::String) {
        self.countries_forbidden = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_countries_forbidden(&mut self) -> &mut ::std::string::String {
        if self.countries_forbidden.is_none() {
            self.countries_forbidden.set_default();
        };
        self.countries_forbidden.as_mut().unwrap()
    }

    // Take field
    pub fn take_countries_forbidden(&mut self) -> ::std::string::String {
        self.countries_forbidden.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_countries_forbidden(&self) -> &str {
        match self.countries_forbidden.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .Restriction.Type typ = 4;

    pub fn clear_typ(&mut self) {
        self.typ = ::std::option::Option::None;
    }

    pub fn has_typ(&self) -> bool {
        self.typ.is_some()
    }

    // Param is passed by value, moved
    pub fn set_typ(&mut self, v: Restriction_Type) {
        self.typ = ::std::option::Option::Some(v);
    }

    pub fn get_typ(&self) -> Restriction_Type {
        self.typ.unwrap_or(Restriction_Type::STREAMING)
    }

    // repeated string catalogue_str = 5;

    pub fn clear_catalogue_str(&mut self) {
        self.catalogue_str.clear();
    }

    // Param is passed by value, moved
    pub fn set_catalogue_str(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.catalogue_str = v;
    }

    // Mutable pointer to the field.
    pub fn mut_catalogue_str(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.catalogue_str
    }

    // Take field
    pub fn take_catalogue_str(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.catalogue_str, ::protobuf::RepeatedField::new())
    }

    pub fn get_catalogue_str(&self) -> &[::std::string::String] {
        &self.catalogue_str
    }
}

impl ::protobuf::Message for Restriction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.countries_allowed));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.countries_forbidden));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.typ = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.catalogue_str));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.countries_allowed {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.countries_forbidden {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.typ {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        for value in &self.catalogue_str {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.countries_allowed.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.countries_forbidden.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.typ {
            try!(os.write_enum(4, v.value()));
        };
        for v in &self.catalogue_str {
            try!(os.write_string(5, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Restriction>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Restriction {
    fn new() -> Restriction {
        Restriction::new()
    }

    fn descriptor_static(_: ::std::option::Option<Restriction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "countries_allowed",
                    Restriction::has_countries_allowed,
                    Restriction::get_countries_allowed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "countries_forbidden",
                    Restriction::has_countries_forbidden,
                    Restriction::get_countries_forbidden,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "typ",
                    Restriction::has_typ,
                    Restriction::get_typ,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "catalogue_str",
                    Restriction::get_catalogue_str,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Restriction>(
                    "Restriction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Restriction {
    fn clear(&mut self) {
        self.clear_countries_allowed();
        self.clear_countries_forbidden();
        self.clear_typ();
        self.clear_catalogue_str();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Restriction {
    fn eq(&self, other: &Restriction) -> bool {
        self.countries_allowed == other.countries_allowed &&
        self.countries_forbidden == other.countries_forbidden &&
        self.typ == other.typ &&
        self.catalogue_str == other.catalogue_str &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Restriction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Restriction_Type {
    STREAMING = 0,
}

impl ::protobuf::ProtobufEnum for Restriction_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Restriction_Type> {
        match value {
            0 => ::std::option::Option::Some(Restriction_Type::STREAMING),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Restriction_Type] = &[
            Restriction_Type::STREAMING,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Restriction_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Restriction_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Restriction_Type {
}

#[derive(Clone,Default)]
pub struct SalePeriod {
    // message fields
    restriction: ::protobuf::RepeatedField<Restriction>,
    start: ::protobuf::SingularPtrField<Date>,
    end: ::protobuf::SingularPtrField<Date>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SalePeriod {}

impl SalePeriod {
    pub fn new() -> SalePeriod {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SalePeriod {
        static mut instance: ::protobuf::lazy::Lazy<SalePeriod> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SalePeriod,
        };
        unsafe {
            instance.get(|| {
                SalePeriod {
                    restriction: ::protobuf::RepeatedField::new(),
                    start: ::protobuf::SingularPtrField::none(),
                    end: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .Restriction restriction = 1;

    pub fn clear_restriction(&mut self) {
        self.restriction.clear();
    }

    // Param is passed by value, moved
    pub fn set_restriction(&mut self, v: ::protobuf::RepeatedField<Restriction>) {
        self.restriction = v;
    }

    // Mutable pointer to the field.
    pub fn mut_restriction(&mut self) -> &mut ::protobuf::RepeatedField<Restriction> {
        &mut self.restriction
    }

    // Take field
    pub fn take_restriction(&mut self) -> ::protobuf::RepeatedField<Restriction> {
        ::std::mem::replace(&mut self.restriction, ::protobuf::RepeatedField::new())
    }

    pub fn get_restriction(&self) -> &[Restriction] {
        &self.restriction
    }

    // optional .Date start = 2;

    pub fn clear_start(&mut self) {
        self.start.clear();
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: Date) {
        self.start = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start(&mut self) -> &mut Date {
        if self.start.is_none() {
            self.start.set_default();
        };
        self.start.as_mut().unwrap()
    }

    // Take field
    pub fn take_start(&mut self) -> Date {
        self.start.take().unwrap_or_else(|| Date::new())
    }

    pub fn get_start(&self) -> &Date {
        self.start.as_ref().unwrap_or_else(|| Date::default_instance())
    }

    // optional .Date end = 3;

    pub fn clear_end(&mut self) {
        self.end.clear();
    }

    pub fn has_end(&self) -> bool {
        self.end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end(&mut self, v: Date) {
        self.end = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end(&mut self) -> &mut Date {
        if self.end.is_none() {
            self.end.set_default();
        };
        self.end.as_mut().unwrap()
    }

    // Take field
    pub fn take_end(&mut self) -> Date {
        self.end.take().unwrap_or_else(|| Date::new())
    }

    pub fn get_end(&self) -> &Date {
        self.end.as_ref().unwrap_or_else(|| Date::default_instance())
    }
}

impl ::protobuf::Message for SalePeriod {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.restriction));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.start));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.end));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.restriction {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.start {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.end {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.restriction {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.start.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.end.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SalePeriod>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SalePeriod {
    fn new() -> SalePeriod {
        SalePeriod::new()
    }

    fn descriptor_static(_: ::std::option::Option<SalePeriod>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "restriction",
                    SalePeriod::get_restriction,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "start",
                    SalePeriod::has_start,
                    SalePeriod::get_start,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "end",
                    SalePeriod::has_end,
                    SalePeriod::get_end,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SalePeriod>(
                    "SalePeriod",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SalePeriod {
    fn clear(&mut self) {
        self.clear_restriction();
        self.clear_start();
        self.clear_end();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SalePeriod {
    fn eq(&self, other: &SalePeriod) -> bool {
        self.restriction == other.restriction &&
        self.start == other.start &&
        self.end == other.end &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SalePeriod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ExternalId {
    // message fields
    typ: ::protobuf::SingularField<::std::string::String>,
    id: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExternalId {}

impl ExternalId {
    pub fn new() -> ExternalId {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExternalId {
        static mut instance: ::protobuf::lazy::Lazy<ExternalId> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExternalId,
        };
        unsafe {
            instance.get(|| {
                ExternalId {
                    typ: ::protobuf::SingularField::none(),
                    id: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string typ = 1;

    pub fn clear_typ(&mut self) {
        self.typ.clear();
    }

    pub fn has_typ(&self) -> bool {
        self.typ.is_some()
    }

    // Param is passed by value, moved
    pub fn set_typ(&mut self, v: ::std::string::String) {
        self.typ = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_typ(&mut self) -> &mut ::std::string::String {
        if self.typ.is_none() {
            self.typ.set_default();
        };
        self.typ.as_mut().unwrap()
    }

    // Take field
    pub fn take_typ(&mut self) -> ::std::string::String {
        self.typ.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_typ(&self) -> &str {
        match self.typ.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string id = 2;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        self.id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        match self.id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for ExternalId {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.typ));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.id));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.typ {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.id {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.typ.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.id.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ExternalId>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExternalId {
    fn new() -> ExternalId {
        ExternalId::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExternalId>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "typ",
                    ExternalId::has_typ,
                    ExternalId::get_typ,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "id",
                    ExternalId::has_id,
                    ExternalId::get_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExternalId>(
                    "ExternalId",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExternalId {
    fn clear(&mut self) {
        self.clear_typ();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExternalId {
    fn eq(&self, other: &ExternalId) -> bool {
        self.typ == other.typ &&
        self.id == other.id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExternalId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AudioFile {
    // message fields
    file_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    format: ::std::option::Option<AudioFile_Format>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AudioFile {}

impl AudioFile {
    pub fn new() -> AudioFile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AudioFile {
        static mut instance: ::protobuf::lazy::Lazy<AudioFile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AudioFile,
        };
        unsafe {
            instance.get(|| {
                AudioFile {
                    file_id: ::protobuf::SingularField::none(),
                    format: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes file_id = 1;

    pub fn clear_file_id(&mut self) {
        self.file_id.clear();
    }

    pub fn has_file_id(&self) -> bool {
        self.file_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.file_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.file_id.is_none() {
            self.file_id.set_default();
        };
        self.file_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_file_id(&mut self) -> ::std::vec::Vec<u8> {
        self.file_id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_file_id(&self) -> &[u8] {
        match self.file_id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .AudioFile.Format format = 2;

    pub fn clear_format(&mut self) {
        self.format = ::std::option::Option::None;
    }

    pub fn has_format(&self) -> bool {
        self.format.is_some()
    }

    // Param is passed by value, moved
    pub fn set_format(&mut self, v: AudioFile_Format) {
        self.format = ::std::option::Option::Some(v);
    }

    pub fn get_format(&self) -> AudioFile_Format {
        self.format.unwrap_or(AudioFile_Format::OGG_VORBIS_96)
    }
}

impl ::protobuf::Message for AudioFile {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.file_id));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.format = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.file_id {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.format {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.file_id.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.format {
            try!(os.write_enum(2, v.value()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AudioFile>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AudioFile {
    fn new() -> AudioFile {
        AudioFile::new()
    }

    fn descriptor_static(_: ::std::option::Option<AudioFile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "file_id",
                    AudioFile::has_file_id,
                    AudioFile::get_file_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "format",
                    AudioFile::has_format,
                    AudioFile::get_format,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AudioFile>(
                    "AudioFile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AudioFile {
    fn clear(&mut self) {
        self.clear_file_id();
        self.clear_format();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AudioFile {
    fn eq(&self, other: &AudioFile) -> bool {
        self.file_id == other.file_id &&
        self.format == other.format &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AudioFile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AudioFile_Format {
    OGG_VORBIS_96 = 0,
    OGG_VORBIS_160 = 1,
    OGG_VORBIS_320 = 2,
    MP3_256 = 3,
    MP3_320 = 4,
    MP3_160 = 5,
    MP3_96 = 6,
    MP3_160_ENC = 7,
    OTHER2 = 8,
    OTHER3 = 9,
    AAC_160 = 10,
    AAC_320 = 11,
}

impl ::protobuf::ProtobufEnum for AudioFile_Format {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AudioFile_Format> {
        match value {
            0 => ::std::option::Option::Some(AudioFile_Format::OGG_VORBIS_96),
            1 => ::std::option::Option::Some(AudioFile_Format::OGG_VORBIS_160),
            2 => ::std::option::Option::Some(AudioFile_Format::OGG_VORBIS_320),
            3 => ::std::option::Option::Some(AudioFile_Format::MP3_256),
            4 => ::std::option::Option::Some(AudioFile_Format::MP3_320),
            5 => ::std::option::Option::Some(AudioFile_Format::MP3_160),
            6 => ::std::option::Option::Some(AudioFile_Format::MP3_96),
            7 => ::std::option::Option::Some(AudioFile_Format::MP3_160_ENC),
            8 => ::std::option::Option::Some(AudioFile_Format::OTHER2),
            9 => ::std::option::Option::Some(AudioFile_Format::OTHER3),
            10 => ::std::option::Option::Some(AudioFile_Format::AAC_160),
            11 => ::std::option::Option::Some(AudioFile_Format::AAC_320),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AudioFile_Format] = &[
            AudioFile_Format::OGG_VORBIS_96,
            AudioFile_Format::OGG_VORBIS_160,
            AudioFile_Format::OGG_VORBIS_320,
            AudioFile_Format::MP3_256,
            AudioFile_Format::MP3_320,
            AudioFile_Format::MP3_160,
            AudioFile_Format::MP3_96,
            AudioFile_Format::MP3_160_ENC,
            AudioFile_Format::OTHER2,
            AudioFile_Format::OTHER3,
            AudioFile_Format::AAC_160,
            AudioFile_Format::AAC_320,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<AudioFile_Format>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AudioFile_Format", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AudioFile_Format {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0e, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x43, 0x0a, 0x09, 0x54, 0x6f, 0x70, 0x54, 0x72, 0x61, 0x63, 0x6b, 0x73, 0x12, 0x18, 0x0a,
    0x07, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x72, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x1c, 0x0a, 0x05, 0x74, 0x72, 0x61, 0x63, 0x6b,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x54, 0x72, 0x61, 0x63, 0x6b, 0x52, 0x05,
    0x74, 0x72, 0x61, 0x63, 0x6b, 0x22, 0x62, 0x0a, 0x0e, 0x41, 0x63, 0x74, 0x69, 0x76, 0x69, 0x74,
    0x79, 0x50, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x74, 0x61, 0x72, 0x74,
    0x5f, 0x79, 0x65, 0x61, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x11, 0x52, 0x09, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x59, 0x65, 0x61, 0x72, 0x12, 0x19, 0x0a, 0x08, 0x65, 0x6e, 0x64, 0x5f, 0x79, 0x65,
    0x61, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x11, 0x52, 0x07, 0x65, 0x6e, 0x64, 0x59, 0x65, 0x61,
    0x72, 0x12, 0x16, 0x0a, 0x06, 0x64, 0x65, 0x63, 0x61, 0x64, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x11, 0x52, 0x06, 0x64, 0x65, 0x63, 0x61, 0x64, 0x65, 0x22, 0xd0, 0x05, 0x0a, 0x06, 0x41, 0x72,
    0x74, 0x69, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x67, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x03, 0x67, 0x69, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1e, 0x0a, 0x0a, 0x70, 0x6f,
    0x70, 0x75, 0x6c, 0x61, 0x72, 0x69, 0x74, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x11, 0x52, 0x0a,
    0x70, 0x6f, 0x70, 0x75, 0x6c, 0x61, 0x72, 0x69, 0x74, 0x79, 0x12, 0x27, 0x0a, 0x09, 0x74, 0x6f,
    0x70, 0x5f, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0a, 0x2e,
    0x54, 0x6f, 0x70, 0x54, 0x72, 0x61, 0x63, 0x6b, 0x73, 0x52, 0x08, 0x74, 0x6f, 0x70, 0x54, 0x72,
    0x61, 0x63, 0x6b, 0x12, 0x2c, 0x0a, 0x0b, 0x61, 0x6c, 0x62, 0x75, 0x6d, 0x5f, 0x67, 0x72, 0x6f,
    0x75, 0x70, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x41, 0x6c, 0x62, 0x75, 0x6d,
    0x47, 0x72, 0x6f, 0x75, 0x70, 0x52, 0x0a, 0x61, 0x6c, 0x62, 0x75, 0x6d, 0x47, 0x72, 0x6f, 0x75,
    0x70, 0x12, 0x2e, 0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x5f, 0x67, 0x72, 0x6f, 0x75,
    0x70, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x41, 0x6c, 0x62, 0x75, 0x6d, 0x47,
    0x72, 0x6f, 0x75, 0x70, 0x52, 0x0b, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x47, 0x72, 0x6f, 0x75,
    0x70, 0x12, 0x38, 0x0a, 0x11, 0x63, 0x6f, 0x6d, 0x70, 0x69, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x5f, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x18, 0x07, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x41,
    0x6c, 0x62, 0x75, 0x6d, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x52, 0x10, 0x63, 0x6f, 0x6d, 0x70, 0x69,
    0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x12, 0x35, 0x0a, 0x10, 0x61,
    0x70, 0x70, 0x65, 0x61, 0x72, 0x73, 0x5f, 0x6f, 0x6e, 0x5f, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x18,
    0x08, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x41, 0x6c, 0x62, 0x75, 0x6d, 0x47, 0x72, 0x6f,
    0x75, 0x70, 0x52, 0x0e, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x73, 0x4f, 0x6e, 0x47, 0x72, 0x6f,
    0x75, 0x70, 0x12, 0x14, 0x0a, 0x05, 0x67, 0x65, 0x6e, 0x72, 0x65, 0x18, 0x09, 0x20, 0x03, 0x28,
    0x09, 0x52, 0x05, 0x67, 0x65, 0x6e, 0x72, 0x65, 0x12, 0x2c, 0x0a, 0x0b, 0x65, 0x78, 0x74, 0x65,
    0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x0a, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0b, 0x2e,
    0x45, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x49, 0x64, 0x52, 0x0a, 0x65, 0x78, 0x74, 0x65,
    0x72, 0x6e, 0x61, 0x6c, 0x49, 0x64, 0x12, 0x22, 0x0a, 0x08, 0x70, 0x6f, 0x72, 0x74, 0x72, 0x61,
    0x69, 0x74, 0x18, 0x0b, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x49, 0x6d, 0x61, 0x67, 0x65,
    0x52, 0x08, 0x70, 0x6f, 0x72, 0x74, 0x72, 0x61, 0x69, 0x74, 0x12, 0x28, 0x0a, 0x09, 0x62, 0x69,
    0x6f, 0x67, 0x72, 0x61, 0x70, 0x68, 0x79, 0x18, 0x0c, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0a, 0x2e,
    0x42, 0x69, 0x6f, 0x67, 0x72, 0x61, 0x70, 0x68, 0x79, 0x52, 0x09, 0x62, 0x69, 0x6f, 0x67, 0x72,
    0x61, 0x70, 0x68, 0x79, 0x12, 0x38, 0x0a, 0x0f, 0x61, 0x63, 0x74, 0x69, 0x76, 0x69, 0x74, 0x79,
    0x5f, 0x70, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x18, 0x0d, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e,
    0x41, 0x63, 0x74, 0x69, 0x76, 0x69, 0x74, 0x79, 0x50, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x52, 0x0e,
    0x61, 0x63, 0x74, 0x69, 0x76, 0x69, 0x74, 0x79, 0x50, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x12, 0x2e,
    0x0a, 0x0b, 0x72, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x0e, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x52, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x52, 0x0b, 0x72, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x21,
    0x0a, 0x07, 0x72, 0x65, 0x6c, 0x61, 0x74, 0x65, 0x64, 0x18, 0x0f, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x07, 0x2e, 0x41, 0x72, 0x74, 0x69, 0x73, 0x74, 0x52, 0x07, 0x72, 0x65, 0x6c, 0x61, 0x74, 0x65,
    0x64, 0x12, 0x35, 0x0a, 0x17, 0x69, 0x73, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x72, 0x61, 0x69, 0x74,
    0x5f, 0x61, 0x6c, 0x62, 0x75, 0x6d, 0x5f, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x18, 0x10, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x14, 0x69, 0x73, 0x50, 0x6f, 0x72, 0x74, 0x72, 0x61, 0x69, 0x74, 0x41, 0x6c,
    0x62, 0x75, 0x6d, 0x43, 0x6f, 0x76, 0x65, 0x72, 0x12, 0x32, 0x0a, 0x0e, 0x70, 0x6f, 0x72, 0x74,
    0x72, 0x61, 0x69, 0x74, 0x5f, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x18, 0x11, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0b, 0x2e, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x52, 0x0d, 0x70,
    0x6f, 0x72, 0x74, 0x72, 0x61, 0x69, 0x74, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x22, 0x2a, 0x0a, 0x0a,
    0x41, 0x6c, 0x62, 0x75, 0x6d, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x12, 0x1c, 0x0a, 0x05, 0x61, 0x6c,
    0x62, 0x75, 0x6d, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x41, 0x6c, 0x62, 0x75,
    0x6d, 0x52, 0x05, 0x61, 0x6c, 0x62, 0x75, 0x6d, 0x22, 0x42, 0x0a, 0x04, 0x44, 0x61, 0x74, 0x65,
    0x12, 0x12, 0x0a, 0x04, 0x79, 0x65, 0x61, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x11, 0x52, 0x04,
    0x79, 0x65, 0x61, 0x72, 0x12, 0x14, 0x0a, 0x05, 0x6d, 0x6f, 0x6e, 0x74, 0x68, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x11, 0x52, 0x05, 0x6d, 0x6f, 0x6e, 0x74, 0x68, 0x12, 0x10, 0x0a, 0x03, 0x64, 0x61,
    0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x11, 0x52, 0x03, 0x64, 0x61, 0x79, 0x22, 0xdb, 0x04, 0x0a,
    0x05, 0x41, 0x6c, 0x62, 0x75, 0x6d, 0x12, 0x10, 0x0a, 0x03, 0x67, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x03, 0x67, 0x69, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1f, 0x0a, 0x06,
    0x61, 0x72, 0x74, 0x69, 0x73, 0x74, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x41,
    0x72, 0x74, 0x69, 0x73, 0x74, 0x52, 0x06, 0x61, 0x72, 0x74, 0x69, 0x73, 0x74, 0x12, 0x1d, 0x0a,
    0x03, 0x74, 0x79, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0b, 0x2e, 0x41, 0x6c, 0x62,
    0x75, 0x6d, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x03, 0x74, 0x79, 0x70, 0x12, 0x14, 0x0a, 0x05,
    0x6c, 0x61, 0x62, 0x65, 0x6c, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x6c, 0x61, 0x62,
    0x65, 0x6c, 0x12, 0x19, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x05, 0x2e, 0x44, 0x61, 0x74, 0x65, 0x52, 0x04, 0x64, 0x61, 0x74, 0x65, 0x12, 0x1e, 0x0a,
    0x0a, 0x70, 0x6f, 0x70, 0x75, 0x6c, 0x61, 0x72, 0x69, 0x74, 0x79, 0x18, 0x07, 0x20, 0x01, 0x28,
    0x11, 0x52, 0x0a, 0x70, 0x6f, 0x70, 0x75, 0x6c, 0x61, 0x72, 0x69, 0x74, 0x79, 0x12, 0x14, 0x0a,
    0x05, 0x67, 0x65, 0x6e, 0x72, 0x65, 0x18, 0x08, 0x20, 0x03, 0x28, 0x09, 0x52, 0x05, 0x67, 0x65,
    0x6e, 0x72, 0x65, 0x12, 0x1c, 0x0a, 0x05, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x18, 0x09, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x06, 0x2e, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x52, 0x05, 0x63, 0x6f, 0x76, 0x65,
    0x72, 0x12, 0x2c, 0x0a, 0x0b, 0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x64,
    0x18, 0x0a, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x45, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x61,
    0x6c, 0x49, 0x64, 0x52, 0x0a, 0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x49, 0x64, 0x12,
    0x19, 0x0a, 0x04, 0x64, 0x69, 0x73, 0x63, 0x18, 0x0b, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x05, 0x2e,
    0x44, 0x69, 0x73, 0x63, 0x52, 0x04, 0x64, 0x69, 0x73, 0x63, 0x12, 0x16, 0x0a, 0x06, 0x72, 0x65,
    0x76, 0x69, 0x65, 0x77, 0x18, 0x0c, 0x20, 0x03, 0x28, 0x09, 0x52, 0x06, 0x72, 0x65, 0x76, 0x69,
    0x65, 0x77, 0x12, 0x28, 0x0a, 0x09, 0x63, 0x6f, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x18,
    0x0d, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x43, 0x6f, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68,
    0x74, 0x52, 0x09, 0x63, 0x6f, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x12, 0x2e, 0x0a, 0x0b,
    0x72, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x0e, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x0c, 0x2e, 0x52, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52,
    0x0b, 0x72, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x20, 0x0a, 0x07,
    0x72, 0x65, 0x6c, 0x61, 0x74, 0x65, 0x64, 0x18, 0x0f, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e,
    0x41, 0x6c, 0x62, 0x75, 0x6d, 0x52, 0x07, 0x72, 0x65, 0x6c, 0x61, 0x74, 0x65, 0x64, 0x12, 0x2c,
    0x0a, 0x0b, 0x73, 0x61, 0x6c, 0x65, 0x5f, 0x70, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x18, 0x10, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x53, 0x61, 0x6c, 0x65, 0x50, 0x65, 0x72, 0x69, 0x6f, 0x64,
    0x52, 0x0a, 0x73, 0x61, 0x6c, 0x65, 0x50, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x12, 0x2c, 0x0a, 0x0b,
    0x63, 0x6f, 0x76, 0x65, 0x72, 0x5f, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x18, 0x11, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0b, 0x2e, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x52, 0x0a,
    0x63, 0x6f, 0x76, 0x65, 0x72, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x22, 0x2e, 0x0a, 0x04, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x09, 0x0a, 0x05, 0x41, 0x4c, 0x42, 0x55, 0x4d, 0x10, 0x01, 0x12, 0x0a, 0x0a,
    0x06, 0x53, 0x49, 0x4e, 0x47, 0x4c, 0x45, 0x10, 0x02, 0x12, 0x0f, 0x0a, 0x0b, 0x43, 0x4f, 0x4d,
    0x50, 0x49, 0x4c, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x10, 0x03, 0x22, 0xf9, 0x03, 0x0a, 0x05, 0x54,
    0x72, 0x61, 0x63, 0x6b, 0x12, 0x10, 0x0a, 0x03, 0x67, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x03, 0x67, 0x69, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1c, 0x0a, 0x05, 0x61, 0x6c,
    0x62, 0x75, 0x6d, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x41, 0x6c, 0x62, 0x75,
    0x6d, 0x52, 0x05, 0x61, 0x6c, 0x62, 0x75, 0x6d, 0x12, 0x1f, 0x0a, 0x06, 0x61, 0x72, 0x74, 0x69,
    0x73, 0x74, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x41, 0x72, 0x74, 0x69, 0x73,
    0x74, 0x52, 0x06, 0x61, 0x72, 0x74, 0x69, 0x73, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x6e, 0x75, 0x6d,
    0x62, 0x65, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x11, 0x52, 0x06, 0x6e, 0x75, 0x6d, 0x62, 0x65,
    0x72, 0x12, 0x1f, 0x0a, 0x0b, 0x64, 0x69, 0x73, 0x63, 0x5f, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x11, 0x52, 0x0a, 0x64, 0x69, 0x73, 0x63, 0x4e, 0x75, 0x6d, 0x62,
    0x65, 0x72, 0x12, 0x1a, 0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x07,
    0x20, 0x01, 0x28, 0x11, 0x52, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1e,
    0x0a, 0x0a, 0x70, 0x6f, 0x70, 0x75, 0x6c, 0x61, 0x72, 0x69, 0x74, 0x79, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x11, 0x52, 0x0a, 0x70, 0x6f, 0x70, 0x75, 0x6c, 0x61, 0x72, 0x69, 0x74, 0x79, 0x12, 0x1a,
    0x0a, 0x08, 0x65, 0x78, 0x70, 0x6c, 0x69, 0x63, 0x69, 0x74, 0x18, 0x09, 0x20, 0x01, 0x28, 0x08,
    0x52, 0x08, 0x65, 0x78, 0x70, 0x6c, 0x69, 0x63, 0x69, 0x74, 0x12, 0x2c, 0x0a, 0x0b, 0x65, 0x78,
    0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x0a, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x0b, 0x2e, 0x45, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x49, 0x64, 0x52, 0x0a, 0x65, 0x78,
    0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x49, 0x64, 0x12, 0x2e, 0x0a, 0x0b, 0x72, 0x65, 0x73, 0x74,
    0x72, 0x69, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x0b, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0c, 0x2e,
    0x52, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0b, 0x72, 0x65, 0x73,
    0x74, 0x72, 0x69, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1e, 0x0a, 0x04, 0x66, 0x69, 0x6c, 0x65,
    0x18, 0x0c, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x46, 0x69,
    0x6c, 0x65, 0x52, 0x04, 0x66, 0x69, 0x6c, 0x65, 0x12, 0x28, 0x0a, 0x0b, 0x61, 0x6c, 0x74, 0x65,
    0x72, 0x6e, 0x61, 0x74, 0x69, 0x76, 0x65, 0x18, 0x0d, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e,
    0x54, 0x72, 0x61, 0x63, 0x6b, 0x52, 0x0b, 0x61, 0x6c, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x74, 0x69,
    0x76, 0x65, 0x12, 0x2c, 0x0a, 0x0b, 0x73, 0x61, 0x6c, 0x65, 0x5f, 0x70, 0x65, 0x72, 0x69, 0x6f,
    0x64, 0x18, 0x0e, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x53, 0x61, 0x6c, 0x65, 0x50, 0x65,
    0x72, 0x69, 0x6f, 0x64, 0x52, 0x0a, 0x73, 0x61, 0x6c, 0x65, 0x50, 0x65, 0x72, 0x69, 0x6f, 0x64,
    0x12, 0x24, 0x0a, 0x07, 0x70, 0x72, 0x65, 0x76, 0x69, 0x65, 0x77, 0x18, 0x0f, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x0a, 0x2e, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x46, 0x69, 0x6c, 0x65, 0x52, 0x07, 0x70,
    0x72, 0x65, 0x76, 0x69, 0x65, 0x77, 0x22, 0xa6, 0x01, 0x0a, 0x05, 0x49, 0x6d, 0x61, 0x67, 0x65,
    0x12, 0x17, 0x0a, 0x07, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x06, 0x66, 0x69, 0x6c, 0x65, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x04, 0x73, 0x69, 0x7a,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0b, 0x2e, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x2e,
    0x53, 0x69, 0x7a, 0x65, 0x52, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x77, 0x69,
    0x64, 0x74, 0x68, 0x18, 0x03, 0x20, 0x01, 0x28, 0x11, 0x52, 0x05, 0x77, 0x69, 0x64, 0x74, 0x68,
    0x12, 0x16, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x11,
    0x52, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x22, 0x35, 0x0a, 0x04, 0x53, 0x69, 0x7a, 0x65,
    0x12, 0x0b, 0x0a, 0x07, 0x44, 0x45, 0x46, 0x41, 0x55, 0x4c, 0x54, 0x10, 0x00, 0x12, 0x09, 0x0a,
    0x05, 0x53, 0x4d, 0x41, 0x4c, 0x4c, 0x10, 0x01, 0x12, 0x09, 0x0a, 0x05, 0x4c, 0x41, 0x52, 0x47,
    0x45, 0x10, 0x02, 0x12, 0x0a, 0x0a, 0x06, 0x58, 0x4c, 0x41, 0x52, 0x47, 0x45, 0x10, 0x03, 0x22,
    0x2a, 0x0a, 0x0a, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x12, 0x1c, 0x0a,
    0x05, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x49,
    0x6d, 0x61, 0x67, 0x65, 0x52, 0x05, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x22, 0x77, 0x0a, 0x09, 0x42,
    0x69, 0x6f, 0x67, 0x72, 0x61, 0x70, 0x68, 0x79, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x74, 0x65, 0x78, 0x74, 0x12, 0x22, 0x0a, 0x08,
    0x70, 0x6f, 0x72, 0x74, 0x72, 0x61, 0x69, 0x74, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06,
    0x2e, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x52, 0x08, 0x70, 0x6f, 0x72, 0x74, 0x72, 0x61, 0x69, 0x74,
    0x12, 0x32, 0x0a, 0x0e, 0x70, 0x6f, 0x72, 0x74, 0x72, 0x61, 0x69, 0x74, 0x5f, 0x67, 0x72, 0x6f,
    0x75, 0x70, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x49, 0x6d, 0x61, 0x67, 0x65,
    0x47, 0x72, 0x6f, 0x75, 0x70, 0x52, 0x0d, 0x70, 0x6f, 0x72, 0x74, 0x72, 0x61, 0x69, 0x74, 0x47,
    0x72, 0x6f, 0x75, 0x70, 0x22, 0x50, 0x0a, 0x04, 0x44, 0x69, 0x73, 0x63, 0x12, 0x16, 0x0a, 0x06,
    0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x11, 0x52, 0x06, 0x6e, 0x75,
    0x6d, 0x62, 0x65, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1c, 0x0a, 0x05, 0x74, 0x72, 0x61, 0x63,
    0x6b, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x54, 0x72, 0x61, 0x63, 0x6b, 0x52,
    0x05, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x22, 0x58, 0x0a, 0x09, 0x43, 0x6f, 0x70, 0x79, 0x72, 0x69,
    0x67, 0x68, 0x74, 0x12, 0x21, 0x0a, 0x03, 0x74, 0x79, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x0f, 0x2e, 0x43, 0x6f, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x2e, 0x54, 0x79, 0x70,
    0x65, 0x52, 0x03, 0x74, 0x79, 0x70, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x74, 0x65, 0x78, 0x74, 0x22, 0x14, 0x0a, 0x04, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x05, 0x0a, 0x01, 0x50, 0x10, 0x00, 0x12, 0x05, 0x0a, 0x01, 0x43, 0x10, 0x01,
    0x22, 0xcc, 0x01, 0x0a, 0x0b, 0x52, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x12, 0x2b, 0x0a, 0x11, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x5f, 0x61, 0x6c,
    0x6c, 0x6f, 0x77, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x10, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x41, 0x6c, 0x6c, 0x6f, 0x77, 0x65, 0x64, 0x12, 0x2f, 0x0a,
    0x13, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x5f, 0x66, 0x6f, 0x72, 0x62, 0x69,
    0x64, 0x64, 0x65, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x12, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x72, 0x69, 0x65, 0x73, 0x46, 0x6f, 0x72, 0x62, 0x69, 0x64, 0x64, 0x65, 0x6e, 0x12, 0x23,
    0x0a, 0x03, 0x74, 0x79, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x11, 0x2e, 0x52, 0x65,
    0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x03,
    0x74, 0x79, 0x70, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x61, 0x74, 0x61, 0x6c, 0x6f, 0x67, 0x75, 0x65,
    0x5f, 0x73, 0x74, 0x72, 0x18, 0x05, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0c, 0x63, 0x61, 0x74, 0x61,
    0x6c, 0x6f, 0x67, 0x75, 0x65, 0x53, 0x74, 0x72, 0x22, 0x15, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65,
    0x12, 0x0d, 0x0a, 0x09, 0x53, 0x54, 0x52, 0x45, 0x41, 0x4d, 0x49, 0x4e, 0x47, 0x10, 0x00, 0x22,
    0x72, 0x0a, 0x0a, 0x53, 0x61, 0x6c, 0x65, 0x50, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x12, 0x2e, 0x0a,
    0x0b, 0x72, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x52, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x52, 0x0b, 0x72, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1b, 0x0a,
    0x05, 0x73, 0x74, 0x61, 0x72, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x44,
    0x61, 0x74, 0x65, 0x52, 0x05, 0x73, 0x74, 0x61, 0x72, 0x74, 0x12, 0x17, 0x0a, 0x03, 0x65, 0x6e,
    0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x44, 0x61, 0x74, 0x65, 0x52, 0x03,
    0x65, 0x6e, 0x64, 0x22, 0x2e, 0x0a, 0x0a, 0x45, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x49,
    0x64, 0x12, 0x10, 0x0a, 0x03, 0x74, 0x79, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03,
    0x74, 0x79, 0x70, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x02, 0x69, 0x64, 0x22, 0x8b, 0x02, 0x0a, 0x09, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x46, 0x69, 0x6c,
    0x65, 0x12, 0x17, 0x0a, 0x07, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x06, 0x66, 0x69, 0x6c, 0x65, 0x49, 0x64, 0x12, 0x29, 0x0a, 0x06, 0x66, 0x6f,
    0x72, 0x6d, 0x61, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x11, 0x2e, 0x41, 0x75, 0x64,
    0x69, 0x6f, 0x46, 0x69, 0x6c, 0x65, 0x2e, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x52, 0x06, 0x66,
    0x6f, 0x72, 0x6d, 0x61, 0x74, 0x22, 0xb9, 0x01, 0x0a, 0x06, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74,
    0x12, 0x11, 0x0a, 0x0d, 0x4f, 0x47, 0x47, 0x5f, 0x56, 0x4f, 0x52, 0x42, 0x49, 0x53, 0x5f, 0x39,
    0x36, 0x10, 0x00, 0x12, 0x12, 0x0a, 0x0e, 0x4f, 0x47, 0x47, 0x5f, 0x56, 0x4f, 0x52, 0x42, 0x49,
    0x53, 0x5f, 0x31, 0x36, 0x30, 0x10, 0x01, 0x12, 0x12, 0x0a, 0x0e, 0x4f, 0x47, 0x47, 0x5f, 0x56,
    0x4f, 0x52, 0x42, 0x49, 0x53, 0x5f, 0x33, 0x32, 0x30, 0x10, 0x02, 0x12, 0x0b, 0x0a, 0x07, 0x4d,
    0x50, 0x33, 0x5f, 0x32, 0x35, 0x36, 0x10, 0x03, 0x12, 0x0b, 0x0a, 0x07, 0x4d, 0x50, 0x33, 0x5f,
    0x33, 0x32, 0x30, 0x10, 0x04, 0x12, 0x0b, 0x0a, 0x07, 0x4d, 0x50, 0x33, 0x5f, 0x31, 0x36, 0x30,
    0x10, 0x05, 0x12, 0x0a, 0x0a, 0x06, 0x4d, 0x50, 0x33, 0x5f, 0x39, 0x36, 0x10, 0x06, 0x12, 0x0f,
    0x0a, 0x0b, 0x4d, 0x50, 0x33, 0x5f, 0x31, 0x36, 0x30, 0x5f, 0x45, 0x4e, 0x43, 0x10, 0x07, 0x12,
    0x0a, 0x0a, 0x06, 0x4f, 0x54, 0x48, 0x45, 0x52, 0x32, 0x10, 0x08, 0x12, 0x0a, 0x0a, 0x06, 0x4f,
    0x54, 0x48, 0x45, 0x52, 0x33, 0x10, 0x09, 0x12, 0x0b, 0x0a, 0x07, 0x41, 0x41, 0x43, 0x5f, 0x31,
    0x36, 0x30, 0x10, 0x0a, 0x12, 0x0b, 0x0a, 0x07, 0x41, 0x41, 0x43, 0x5f, 0x33, 0x32, 0x30, 0x10,
    0x0b, 0x4a, 0xa2, 0x39, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xa2, 0x01, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02,
    0x00, 0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x11, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x03, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x03, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x03, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x03, 0x1e, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04, 0x04,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x04, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x04, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x1b, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x07, 0x00, 0x0b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x07, 0x08,
    0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x08, 0x04, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x08, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x08, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x14, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x08, 0x21, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03,
    0x09, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x09, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x09, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x09, 0x14, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x09, 0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x02, 0x12, 0x03, 0x0a, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x0a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x0a, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0a,
    0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0a, 0x1d, 0x20,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0d, 0x00, 0x1f, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00,
    0x12, 0x03, 0x0e, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x0e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0e, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x13, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x19, 0x1c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x0f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x0f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x0f, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0f,
    0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x10, 0x04, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x10, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x10, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x14, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x10, 0x21, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x11, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x11,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x06, 0x12, 0x03, 0x11, 0x0d, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x11, 0x17, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x11, 0x23, 0x26, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x12, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x12, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x06,
    0x12, 0x03, 0x12, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x12, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x12, 0x26,
    0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x13, 0x04, 0x2b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x04, 0x12, 0x03, 0x13, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x05, 0x06, 0x12, 0x03, 0x13, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x13, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x13, 0x27, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x14, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x04, 0x12, 0x03, 0x14, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x06, 0x12, 0x03, 0x14, 0x0d, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x14, 0x18, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x06, 0x03, 0x12, 0x03, 0x14, 0x2c, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x07, 0x12, 0x03, 0x15, 0x04, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07,
    0x04, 0x12, 0x03, 0x15, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x06, 0x12,
    0x03, 0x15, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03, 0x15,
    0x18, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x03, 0x12, 0x03, 0x15, 0x2b, 0x2e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x08, 0x12, 0x03, 0x16, 0x04, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x08, 0x04, 0x12, 0x03, 0x16, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x08, 0x05, 0x12, 0x03, 0x16, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x08, 0x01, 0x12, 0x03, 0x16, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x03,
    0x12, 0x03, 0x16, 0x1c, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x09, 0x12, 0x03, 0x17,
    0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x04, 0x12, 0x03, 0x17, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x06, 0x12, 0x03, 0x17, 0x0d, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x01, 0x12, 0x03, 0x17, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x09, 0x03, 0x12, 0x03, 0x17, 0x26, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x0a, 0x12, 0x03, 0x18, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x04,
    0x12, 0x03, 0x18, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x06, 0x12, 0x03,
    0x18, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x18, 0x13,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x18, 0x1e, 0x21, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x0b, 0x12, 0x03, 0x19, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x19, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x0b, 0x06, 0x12, 0x03, 0x19, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0b,
    0x01, 0x12, 0x03, 0x19, 0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0b, 0x03, 0x12,
    0x03, 0x19, 0x23, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x0c, 0x12, 0x03, 0x1a, 0x04,
    0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x1a, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0c, 0x06, 0x12, 0x03, 0x1a, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x1a, 0x1c, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x1a, 0x2e, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x0d, 0x12, 0x03, 0x1b, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0d, 0x04, 0x12,
    0x03, 0x1b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0d, 0x06, 0x12, 0x03, 0x1b,
    0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x1b, 0x19, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x1b, 0x27, 0x2a, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x0e, 0x12, 0x03, 0x1c, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x1c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x0e, 0x06, 0x12, 0x03, 0x1c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0e, 0x01,
    0x12, 0x03, 0x1c, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0e, 0x03, 0x12, 0x03,
    0x1c, 0x1e, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x0f, 0x12, 0x03, 0x1d, 0x04, 0x31,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0f, 0x04, 0x12, 0x03, 0x1d, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x0f, 0x05, 0x12, 0x03, 0x1d, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x1d, 0x12, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x0f, 0x03, 0x12, 0x03, 0x1d, 0x2c, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x10,
    0x12, 0x03, 0x1e, 0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x10, 0x04, 0x12, 0x03,
    0x1e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x10, 0x06, 0x12, 0x03, 0x1e, 0x0d,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x10, 0x01, 0x12, 0x03, 0x1e, 0x18, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x10, 0x03, 0x12, 0x03, 0x1e, 0x29, 0x2d, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x03, 0x12, 0x04, 0x21, 0x00, 0x23, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01,
    0x12, 0x03, 0x21, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x22,
    0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x22, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x22, 0x0d, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x22, 0x1b, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x25, 0x00, 0x29, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x25,
    0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x26, 0x04, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x26, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x26, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x26, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x26, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x27, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x27,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x27, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x27, 0x14, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x27, 0x1c, 0x1f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x28, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x28, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x28, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x28, 0x14, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x28, 0x1a,
    0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x2b, 0x00, 0x42, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x2b, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x00, 0x12, 0x03, 0x2c, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x2c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2c,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2c, 0x13, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x19, 0x1c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x2d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x2d, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x2d, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x2e, 0x04, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2e, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2e, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x2e, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03,
    0x12, 0x03, 0x2f, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x2f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x06, 0x12, 0x03, 0x2f, 0x0d,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2f, 0x12, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2f, 0x18, 0x1b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x05, 0x04, 0x00, 0x12, 0x04, 0x30, 0x04, 0x34, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x04, 0x00, 0x01, 0x12, 0x03, 0x30, 0x09, 0x0d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x31, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x31, 0x08, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x31, 0x10, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x32, 0x08, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x32, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x32, 0x11, 0x14, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x33, 0x08, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x33, 0x08, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x33, 0x16, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x04, 0x12, 0x03,
    0x35, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x04, 0x12, 0x03, 0x35, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x05, 0x12, 0x03, 0x35, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x01, 0x12, 0x03, 0x35, 0x14, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x04, 0x03, 0x12, 0x03, 0x35, 0x1c, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x05, 0x12, 0x03, 0x36, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05,
    0x04, 0x12, 0x03, 0x36, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x06, 0x12,
    0x03, 0x36, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x01, 0x12, 0x03, 0x36,
    0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x03, 0x12, 0x03, 0x36, 0x19, 0x1c,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x06, 0x12, 0x03, 0x37, 0x04, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x06, 0x04, 0x12, 0x03, 0x37, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x06, 0x05, 0x12, 0x03, 0x37, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x06, 0x01, 0x12, 0x03, 0x37, 0x14, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x03,
    0x12, 0x03, 0x37, 0x21, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x07, 0x12, 0x03, 0x38,
    0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x04, 0x12, 0x03, 0x38, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x05, 0x12, 0x03, 0x38, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x01, 0x12, 0x03, 0x38, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x07, 0x03, 0x12, 0x03, 0x38, 0x1c, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x08, 0x12, 0x03, 0x39, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x08, 0x04,
    0x12, 0x03, 0x39, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x08, 0x06, 0x12, 0x03,
    0x39, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x08, 0x01, 0x12, 0x03, 0x39, 0x13,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x08, 0x03, 0x12, 0x03, 0x39, 0x1b, 0x1e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x09, 0x12, 0x03, 0x3a, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x09, 0x04, 0x12, 0x03, 0x3a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x09, 0x06, 0x12, 0x03, 0x3a, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x09,
    0x01, 0x12, 0x03, 0x3a, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x09, 0x03, 0x12,
    0x03, 0x3a, 0x26, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x0a, 0x12, 0x03, 0x3b, 0x04,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x3b, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0a, 0x06, 0x12, 0x03, 0x3b, 0x0d, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x3b, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x3b, 0x19, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x0b, 0x12, 0x03, 0x3c, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0b, 0x04, 0x12,
    0x03, 0x3c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x3c,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x3c, 0x14, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x3c, 0x1d, 0x20, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x0c, 0x12, 0x03, 0x3d, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x3d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x0c, 0x06, 0x12, 0x03, 0x3d, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0c, 0x01,
    0x12, 0x03, 0x3d, 0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0c, 0x03, 0x12, 0x03,
    0x3d, 0x23, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x0d, 0x12, 0x03, 0x3e, 0x04, 0x2b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x3e, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x0d, 0x06, 0x12, 0x03, 0x3e, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x3e, 0x19, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x0d, 0x03, 0x12, 0x03, 0x3e, 0x27, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x0e,
    0x12, 0x03, 0x3f, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0e, 0x04, 0x12, 0x03,
    0x3f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0e, 0x06, 0x12, 0x03, 0x3f, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x3f, 0x13, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x3f, 0x1d, 0x20, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x0f, 0x12, 0x03, 0x40, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x0f, 0x04, 0x12, 0x03, 0x40, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0f,
    0x06, 0x12, 0x03, 0x40, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0f, 0x01, 0x12,
    0x03, 0x40, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x40,
    0x26, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x10, 0x12, 0x03, 0x41, 0x04, 0x2b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x10, 0x04, 0x12, 0x03, 0x41, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x10, 0x06, 0x12, 0x03, 0x41, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x10, 0x01, 0x12, 0x03, 0x41, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x10, 0x03, 0x12, 0x03, 0x41, 0x26, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x44,
    0x00, 0x54, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x44, 0x08, 0x0d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x45, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x45, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x45, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x45, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x45, 0x19, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x46, 0x04,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x46, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x46, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x46, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x46, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x02, 0x12, 0x03, 0x47, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x47, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x06, 0x12, 0x03, 0x47,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x47, 0x13, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x47, 0x1b, 0x1e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x48, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x48, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x48, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x48, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x48, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x03, 0x49, 0x04, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x04, 0x12, 0x03, 0x49, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x05, 0x12, 0x03, 0x49, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x49, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x49, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x05,
    0x12, 0x03, 0x4a, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x4a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x05, 0x12, 0x03, 0x4a, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x01, 0x12, 0x03, 0x4a, 0x14, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12, 0x03, 0x4a, 0x22, 0x25, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x06, 0x12, 0x03, 0x4b, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x06, 0x04, 0x12, 0x03, 0x4b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x4b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x4b, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x03, 0x12, 0x03, 0x4b,
    0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x07, 0x12, 0x03, 0x4c, 0x04, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x04, 0x12, 0x03, 0x4c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x07, 0x05, 0x12, 0x03, 0x4c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x07, 0x01, 0x12, 0x03, 0x4c, 0x14, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x07, 0x03, 0x12, 0x03, 0x4c, 0x21, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x08, 0x12,
    0x03, 0x4d, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x04, 0x12, 0x03, 0x4d,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x05, 0x12, 0x03, 0x4d, 0x0d, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x01, 0x12, 0x03, 0x4d, 0x12, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x03, 0x12, 0x03, 0x4d, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x09, 0x12, 0x03, 0x4e, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x09, 0x04, 0x12, 0x03, 0x4e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x09, 0x06,
    0x12, 0x03, 0x4e, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x09, 0x01, 0x12, 0x03,
    0x4e, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x09, 0x03, 0x12, 0x03, 0x4e, 0x26,
    0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x0a, 0x12, 0x03, 0x4f, 0x04, 0x2b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x4f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x0a, 0x06, 0x12, 0x03, 0x4f, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x0a, 0x01, 0x12, 0x03, 0x4f, 0x19, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0a,
    0x03, 0x12, 0x03, 0x4f, 0x27, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x0b, 0x12, 0x03,
    0x50, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x50, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0b, 0x06, 0x12, 0x03, 0x50, 0x0d, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x50, 0x17, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x50, 0x1e, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x0c, 0x12, 0x03, 0x51, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0c,
    0x04, 0x12, 0x03, 0x51, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0c, 0x06, 0x12,
    0x03, 0x51, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x51,
    0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x51, 0x21, 0x24,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x0d, 0x12, 0x03, 0x52, 0x04, 0x2a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x52, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x0d, 0x06, 0x12, 0x03, 0x52, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x0d, 0x01, 0x12, 0x03, 0x52, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0d, 0x03,
    0x12, 0x03, 0x52, 0x26, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x0e, 0x12, 0x03, 0x53,
    0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x53, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0e, 0x06, 0x12, 0x03, 0x53, 0x0d, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x53, 0x17, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x53, 0x21, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07,
    0x12, 0x04, 0x56, 0x00, 0x61, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x56,
    0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x57, 0x04, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x57, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x57, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x57, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x57, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12,
    0x03, 0x58, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x58,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x06, 0x12, 0x03, 0x58, 0x0d, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x58, 0x12, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x58, 0x19, 0x1c, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x07, 0x04, 0x00, 0x12, 0x04, 0x59, 0x04, 0x5e, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x59, 0x09, 0x0d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x5a, 0x08, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x5a, 0x08, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x03, 0x5a, 0x12, 0x15, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x5b, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x5b, 0x08, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x5b, 0x10, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x5c, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x5c, 0x08, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x5c, 0x10, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x03, 0x12,
    0x03, 0x5d, 0x08, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x5d, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12,
    0x03, 0x5d, 0x11, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x03, 0x5f, 0x04,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x03, 0x5f, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x05, 0x12, 0x03, 0x5f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5f, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5f, 0x1c, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x03, 0x12, 0x03, 0x60, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x60, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x05, 0x12, 0x03, 0x60,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x03, 0x60, 0x14, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03, 0x12, 0x03, 0x60, 0x1d, 0x20, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x63, 0x00, 0x65, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08,
    0x01, 0x12, 0x03, 0x63, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03,
    0x64, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x64, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x64, 0x0d, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x64, 0x13, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x64, 0x1b, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x09, 0x12, 0x04, 0x67, 0x00, 0x6b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03,
    0x67, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x68, 0x04, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x68, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x68, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x68, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x68, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01,
    0x12, 0x03, 0x69, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x69, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x06, 0x12, 0x03, 0x69, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x69, 0x13, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x69, 0x1e, 0x21, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x03, 0x6a, 0x04, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x6a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x6a, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x6a, 0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6a,
    0x29, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x6d, 0x00, 0x71, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x6d, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a,
    0x02, 0x00, 0x12, 0x03, 0x6e, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x6e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x6e, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6e, 0x14,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6e, 0x1d, 0x20, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x03, 0x6f, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x6f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x6f, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x6f, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x02, 0x12, 0x03, 0x70, 0x04,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x04, 0x12, 0x03, 0x70, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x06, 0x12, 0x03, 0x70, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x02, 0x01, 0x12, 0x03, 0x70, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x02, 0x03, 0x12, 0x03, 0x70, 0x1b, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12,
    0x04, 0x73, 0x00, 0x7a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x73, 0x08,
    0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x74, 0x04, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x74, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x00, 0x06, 0x12, 0x03, 0x74, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x74, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x74, 0x18, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x04, 0x00, 0x12, 0x04,
    0x75, 0x04, 0x78, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x04, 0x00, 0x01, 0x12, 0x03, 0x75,
    0x09, 0x0d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0b, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x76, 0x08,
    0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x76, 0x08,
    0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x76, 0x0c,
    0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0b, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x77, 0x08, 0x10,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x77, 0x08, 0x09,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x77, 0x0c, 0x0f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x79, 0x04, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x03, 0x79, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x01, 0x05, 0x12, 0x03, 0x79, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x79, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x79, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x05, 0x7c, 0x00, 0x84,
    0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x7c, 0x08, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x7d, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x7d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x7d, 0x14, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x7d, 0x28, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x7e, 0x04, 0x2e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7e, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x05, 0x12, 0x03, 0x7e, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7e, 0x14, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x7e, 0x2a, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02,
    0x12, 0x03, 0x7f, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x7f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x06, 0x12, 0x03, 0x7f, 0x0d,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x03, 0x7f, 0x12, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x03, 0x7f, 0x18, 0x1b, 0x0a, 0x0e, 0x0a,
    0x04, 0x04, 0x0c, 0x04, 0x00, 0x12, 0x06, 0x80, 0x01, 0x04, 0x82, 0x01, 0x05, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x04, 0x00, 0x01, 0x12, 0x04, 0x80, 0x01, 0x09, 0x0d, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x0c, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0x81, 0x01, 0x08, 0x18, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x0c, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x81, 0x01, 0x08, 0x11, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0x81, 0x01, 0x14, 0x17, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0c, 0x02, 0x03, 0x12, 0x04, 0x83, 0x01, 0x04, 0x28, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x03, 0x04, 0x12, 0x04, 0x83, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x03, 0x05, 0x12, 0x04, 0x83, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x03, 0x01, 0x12, 0x04, 0x83, 0x01, 0x14, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x03, 0x03, 0x12, 0x04, 0x83, 0x01, 0x24, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x06,
    0x86, 0x01, 0x00, 0x8a, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x04, 0x86,
    0x01, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x04, 0x87, 0x01, 0x04,
    0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x04, 0x87, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x06, 0x12, 0x04, 0x87, 0x01, 0x0d, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0x87, 0x01, 0x19, 0x24, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0x87, 0x01, 0x27, 0x2a, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x04, 0x88, 0x01, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x01, 0x04, 0x12, 0x04, 0x88, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x01, 0x06, 0x12, 0x04, 0x88, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x88, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01,
    0x03, 0x12, 0x04, 0x88, 0x01, 0x1a, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x02, 0x12,
    0x04, 0x89, 0x01, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x89, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x06, 0x12, 0x04, 0x89,
    0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x01, 0x12, 0x04, 0x89, 0x01,
    0x12, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x03, 0x12, 0x04, 0x89, 0x01, 0x18,
    0x1b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0x8c, 0x01, 0x00, 0x8f, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0x8d, 0x01, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x8d, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x8d, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x8d, 0x01, 0x14, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x8d, 0x01, 0x1a, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01, 0x12, 0x04,
    0x8e, 0x01, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8e,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8e, 0x01,
    0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x14,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8e, 0x01, 0x19, 0x1c,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0x91, 0x01, 0x00, 0xa2, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0x91, 0x01, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0f, 0x02, 0x00, 0x12, 0x04, 0x92, 0x01, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x92, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00,
    0x05, 0x12, 0x04, 0x92, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x92, 0x01, 0x13, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x92, 0x01, 0x1d, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x04, 0x93,
    0x01, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x04, 0x93, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x06, 0x12, 0x04, 0x93, 0x01, 0x0d,
    0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12, 0x04, 0x93, 0x01, 0x14, 0x1a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x04, 0x93, 0x01, 0x1d, 0x20, 0x0a,
    0x0e, 0x0a, 0x04, 0x04, 0x0f, 0x04, 0x00, 0x12, 0x06, 0x94, 0x01, 0x04, 0xa1, 0x01, 0x05, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x04, 0x00, 0x01, 0x12, 0x04, 0x94, 0x01, 0x09, 0x0f, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0x95, 0x01, 0x08, 0x1c, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x95, 0x01, 0x08, 0x15, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0x95, 0x01, 0x18, 0x1b,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0x96, 0x01, 0x08, 0x1d,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x96, 0x01, 0x08,
    0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0x96, 0x01,
    0x19, 0x1c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0x97, 0x01,
    0x08, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0x97,
    0x01, 0x08, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04,
    0x97, 0x01, 0x19, 0x1c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04,
    0x98, 0x01, 0x08, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x04, 0x98, 0x01, 0x08, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x03, 0x02,
    0x12, 0x04, 0x98, 0x01, 0x12, 0x15, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x04,
    0x12, 0x04, 0x99, 0x01, 0x08, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x99, 0x01, 0x08, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02,
    0x04, 0x02, 0x12, 0x04, 0x99, 0x01, 0x12, 0x15, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00,
    0x02, 0x05, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00,
    0x02, 0x05, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x04,
    0x00, 0x02, 0x05, 0x02, 0x12, 0x04, 0x9a, 0x01, 0x12, 0x15, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f,
    0x04, 0x00, 0x02, 0x06, 0x12, 0x04, 0x9b, 0x01, 0x08, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f,
    0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x08, 0x0e, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x0f, 0x04, 0x00, 0x02, 0x06, 0x02, 0x12, 0x04, 0x9b, 0x01, 0x11, 0x14, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x0f, 0x04, 0x00, 0x02, 0x07, 0x12, 0x04, 0x9c, 0x01, 0x08, 0x1a, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x0f, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x04, 0x9c, 0x01, 0x08, 0x13, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x07, 0x02, 0x12, 0x04, 0x9c, 0x01, 0x16, 0x19, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x08, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x15, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x0e, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x08, 0x02, 0x12, 0x04, 0x9d, 0x01, 0x11, 0x14,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x09, 0x12, 0x04, 0x9e, 0x01, 0x08, 0x15,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x08,
    0x0e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x09, 0x02, 0x12, 0x04, 0x9e, 0x01,
    0x11, 0x14, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x04, 0x9f, 0x01,
    0x08, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x04, 0x9f,
    0x01, 0x08, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x04,
    0x9f, 0x01, 0x12, 0x15, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x04,
    0xa0, 0x01, 0x08, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12,
    0x04, 0xa0, 0x01, 0x08, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x0b, 0x02,
    0x12, 0x04, 0xa0, 0x01, 0x12, 0x15,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
