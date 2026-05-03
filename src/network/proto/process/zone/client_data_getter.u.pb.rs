const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.32.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__EntityData_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct EntityData {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<EntityData>
}

impl ::protobuf::Message for EntityData {}

impl ::std::default::Default for EntityData {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for EntityData {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for EntityData {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for EntityData {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `EntityData` is `Sync` because it does not implement interior mutability.
//    Neither does `EntityDataMut`.
unsafe impl Sync for EntityData {}

// SAFETY:
// - `EntityData` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for EntityData {}

impl ::protobuf::Proxied for EntityData {
  type View<'msg> = EntityDataView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for EntityData {}

impl ::protobuf::MutProxied for EntityData {
  type Mut<'msg> = EntityDataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EntityDataView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EntityData>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EntityDataView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EntityDataView<'msg> {
  type Message = EntityData;
}

impl ::std::fmt::Debug for EntityDataView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for EntityDataView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for EntityDataView<'_> {
  fn default() -> EntityDataView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    EntityDataView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> EntityDataView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EntityData>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> EntityData {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // entity_id: optional uint64
  pub fn entity_id(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // position: optional message Proto.Vector3
  pub fn has_position(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn position_opt(self) -> ::protobuf::Optional<super::Vector3View<'msg>> {
        ::protobuf::Optional::new(self.position(), self.has_position())
  }
  pub fn position(self) -> super::Vector3View<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::Vector3View::new(::protobuf::__internal::Private, inner)
  }

  // hp: optional int32
  pub fn hp(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `EntityDataView` is `Sync` because it does not support mutation.
unsafe impl Sync for EntityDataView<'_> {}

// SAFETY:
// - `EntityDataView` is `Send` because while its alive a `EntityDataMut` cannot.
// - `EntityDataView` does not use thread-local data.
unsafe impl Send for EntityDataView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for EntityDataView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for EntityDataView<'msg> {}

impl<'msg> ::protobuf::AsView for EntityDataView<'msg> {
  type Proxied = EntityData;
  fn as_view(&self) -> ::protobuf::View<'msg, EntityData> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EntityDataView<'msg> {
  fn into_view<'shorter>(self) -> EntityDataView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<EntityData> for EntityDataView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EntityData {
    let mut dst = EntityData::new();
    let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(&mut dst, ::protobuf::__internal::Private);
    let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut dst, ::protobuf::__internal::Private);
    let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(&self, ::protobuf::__internal::Private);

    unsafe { ::protobuf::__internal::runtime::upb_Message_DeepCopy(
      dst_raw,
      src_raw,
      <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      dst_arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<EntityData> for EntityDataMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EntityData {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for EntityData {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn into_message_value_fuse_if_required(
      raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      mut val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let parent_arena = ::std::mem::ManuallyDrop::new(
          unsafe { ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena) });

      parent_arena.fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
      ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn from_message_value<'msg>(msg: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
        EntityDataView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> EntityDataMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, EntityData>::wrap_raw(msg, arena) };
        EntityDataMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EntityDataMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EntityData>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EntityDataMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EntityDataMut<'msg> {
  type Message = EntityData;
}

impl ::std::fmt::Debug for EntityDataMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for EntityDataMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> EntityDataMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EntityData>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, EntityData> {
    self.inner
  }

  pub fn to_owned(&self) -> EntityData {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // entity_id: optional uint64
  pub fn entity_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_entity_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

  // position: optional message Proto.Vector3
  pub fn has_position(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_position(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn position_opt(&self) -> ::protobuf::Optional<super::Vector3View<'_>> {
        ::protobuf::Optional::new(self.position(), self.has_position())
  }
  pub fn position(&self) -> super::Vector3View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::Vector3View::new(::protobuf::__internal::Private, inner)
  }
  pub fn position_mut(&mut self) -> super::Vector3Mut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.arena()
       ).unwrap()
     };
     super::Vector3Mut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_position(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        1, child_ptr
      );
    }
  }

  // hp: optional int32
  pub fn hp(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_hp(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

}

// SAFETY:
// - `EntityDataMut` does not perform any shared mutation.
// - `EntityDataMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for EntityDataMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for EntityDataMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for EntityDataMut<'msg> {}

impl<'msg> ::protobuf::AsView for EntityDataMut<'msg> {
  type Proxied = EntityData;
  fn as_view(&self) -> ::protobuf::View<'_, EntityData> {
    EntityDataView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EntityDataMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, EntityData>
  where
      'msg: 'shorter {
    EntityDataView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for EntityDataMut<'msg> {
  type MutProxied = EntityData;
  fn as_mut(&mut self) -> EntityDataMut<'msg> {
    EntityDataMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EntityDataMut<'msg> {
  fn into_mut<'shorter>(self) -> EntityDataMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl EntityData {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, EntityData> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> EntityDataView {
    EntityDataView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> EntityDataMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    EntityDataMut::new(::protobuf::__internal::Private, inner)
  }

  // entity_id: optional uint64
  pub fn entity_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_entity_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

  // position: optional message Proto.Vector3
  pub fn has_position(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_position(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn position_opt(&self) -> ::protobuf::Optional<super::Vector3View<'_>> {
        ::protobuf::Optional::new(self.position(), self.has_position())
  }
  pub fn position(&self) -> super::Vector3View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::Vector3View::new(::protobuf::__internal::Private, inner)
  }
  pub fn position_mut(&mut self) -> super::Vector3Mut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.arena()
       ).unwrap()
     };
     super::Vector3Mut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_position(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        1, child_ptr
      );
    }
  }

  // hp: optional int32
  pub fn hp(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_hp(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

}  // impl EntityData

impl ::std::ops::Drop for EntityData {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for EntityData {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for EntityData {
  type Proxied = Self;
  fn as_view(&self) -> EntityDataView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for EntityData {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EntityDataMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EntityData {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__EntityData_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$,Pa3(P".as_ptr(),
              7,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
        <super::Vector3 as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__EntityData_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__EntityData_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EntityData {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EntityDataView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <EntityData as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EntityDataMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <EntityData as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EntityData {
  type Msg = EntityData;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EntityData> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EntityData {
  type Msg = EntityData;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EntityData> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EntityDataMut<'_> {
  type Msg = EntityData;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EntityData> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EntityDataMut<'_> {
  type Msg = EntityData;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EntityData> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EntityDataView<'_> {
  type Msg = EntityData;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EntityData> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EntityDataMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for EntityData {}
impl<'a> ::protobuf::MessageMutInterop<'a> for EntityDataMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for EntityDataView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__PlayerData_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PlayerData {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PlayerData>
}

impl ::protobuf::Message for PlayerData {}

impl ::std::default::Default for PlayerData {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PlayerData {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PlayerData {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PlayerData {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PlayerData` is `Sync` because it does not implement interior mutability.
//    Neither does `PlayerDataMut`.
unsafe impl Sync for PlayerData {}

// SAFETY:
// - `PlayerData` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PlayerData {}

impl ::protobuf::Proxied for PlayerData {
  type View<'msg> = PlayerDataView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PlayerData {}

impl ::protobuf::MutProxied for PlayerData {
  type Mut<'msg> = PlayerDataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PlayerDataView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PlayerData>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PlayerDataView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PlayerDataView<'msg> {
  type Message = PlayerData;
}

impl ::std::fmt::Debug for PlayerDataView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PlayerDataView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PlayerDataView<'_> {
  fn default() -> PlayerDataView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PlayerDataView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PlayerDataView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PlayerData>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PlayerData {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // entity_data: optional message Proto.EntityData
  pub fn has_entity_data(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn entity_data_opt(self) -> ::protobuf::Optional<super::EntityDataView<'msg>> {
        ::protobuf::Optional::new(self.entity_data(), self.has_entity_data())
  }
  pub fn entity_data(self) -> super::EntityDataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::EntityDataView::new(::protobuf::__internal::Private, inner)
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `PlayerDataView` is `Sync` because it does not support mutation.
unsafe impl Sync for PlayerDataView<'_> {}

// SAFETY:
// - `PlayerDataView` is `Send` because while its alive a `PlayerDataMut` cannot.
// - `PlayerDataView` does not use thread-local data.
unsafe impl Send for PlayerDataView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PlayerDataView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PlayerDataView<'msg> {}

impl<'msg> ::protobuf::AsView for PlayerDataView<'msg> {
  type Proxied = PlayerData;
  fn as_view(&self) -> ::protobuf::View<'msg, PlayerData> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PlayerDataView<'msg> {
  fn into_view<'shorter>(self) -> PlayerDataView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PlayerData> for PlayerDataView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PlayerData {
    let mut dst = PlayerData::new();
    let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(&mut dst, ::protobuf::__internal::Private);
    let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut dst, ::protobuf::__internal::Private);
    let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(&self, ::protobuf::__internal::Private);

    unsafe { ::protobuf::__internal::runtime::upb_Message_DeepCopy(
      dst_raw,
      src_raw,
      <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      dst_arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PlayerData> for PlayerDataMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PlayerData {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PlayerData {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn into_message_value_fuse_if_required(
      raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      mut val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let parent_arena = ::std::mem::ManuallyDrop::new(
          unsafe { ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena) });

      parent_arena.fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
      ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn from_message_value<'msg>(msg: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
        PlayerDataView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PlayerDataMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PlayerData>::wrap_raw(msg, arena) };
        PlayerDataMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PlayerDataMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PlayerData>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PlayerDataMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PlayerDataMut<'msg> {
  type Message = PlayerData;
}

impl ::std::fmt::Debug for PlayerDataMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PlayerDataMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PlayerDataMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PlayerData>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PlayerData> {
    self.inner
  }

  pub fn to_owned(&self) -> PlayerData {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // entity_data: optional message Proto.EntityData
  pub fn has_entity_data(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_entity_data(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn entity_data_opt(&self) -> ::protobuf::Optional<super::EntityDataView<'_>> {
        ::protobuf::Optional::new(self.entity_data(), self.has_entity_data())
  }
  pub fn entity_data(&self) -> super::EntityDataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::EntityDataView::new(::protobuf::__internal::Private, inner)
  }
  pub fn entity_data_mut(&mut self) -> super::EntityDataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.arena()
       ).unwrap()
     };
     super::EntityDataMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_entity_data(&mut self,
    val: impl ::protobuf::IntoProxied<super::EntityData>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        0, child_ptr
      );
    }
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        1,
        view,
      );
    }
  }

}

// SAFETY:
// - `PlayerDataMut` does not perform any shared mutation.
// - `PlayerDataMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PlayerDataMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PlayerDataMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PlayerDataMut<'msg> {}

impl<'msg> ::protobuf::AsView for PlayerDataMut<'msg> {
  type Proxied = PlayerData;
  fn as_view(&self) -> ::protobuf::View<'_, PlayerData> {
    PlayerDataView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PlayerDataMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PlayerData>
  where
      'msg: 'shorter {
    PlayerDataView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PlayerDataMut<'msg> {
  type MutProxied = PlayerData;
  fn as_mut(&mut self) -> PlayerDataMut<'msg> {
    PlayerDataMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PlayerDataMut<'msg> {
  fn into_mut<'shorter>(self) -> PlayerDataMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PlayerData {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PlayerData> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> PlayerDataView {
    PlayerDataView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PlayerDataMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PlayerDataMut::new(::protobuf::__internal::Private, inner)
  }

  // entity_data: optional message Proto.EntityData
  pub fn has_entity_data(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_entity_data(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn entity_data_opt(&self) -> ::protobuf::Optional<super::EntityDataView<'_>> {
        ::protobuf::Optional::new(self.entity_data(), self.has_entity_data())
  }
  pub fn entity_data(&self) -> super::EntityDataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::EntityDataView::new(::protobuf::__internal::Private, inner)
  }
  pub fn entity_data_mut(&mut self) -> super::EntityDataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.arena()
       ).unwrap()
     };
     super::EntityDataMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_entity_data(&mut self,
    val: impl ::protobuf::IntoProxied<super::EntityData>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        0, child_ptr
      );
    }
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        1,
        view,
      );
    }
  }

}  // impl PlayerData

impl ::std::ops::Drop for PlayerData {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PlayerData {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PlayerData {
  type Proxied = Self;
  fn as_view(&self) -> PlayerDataView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PlayerData {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PlayerDataMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PlayerData {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PlayerData_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$31X".as_ptr(),
              4,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
        <super::EntityData as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__PlayerData_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PlayerData_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PlayerData {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PlayerDataView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PlayerData as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PlayerDataMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PlayerData as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PlayerData {
  type Msg = PlayerData;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PlayerData> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PlayerData {
  type Msg = PlayerData;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PlayerData> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PlayerDataMut<'_> {
  type Msg = PlayerData;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PlayerData> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PlayerDataMut<'_> {
  type Msg = PlayerData;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PlayerData> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PlayerDataView<'_> {
  type Msg = PlayerData;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PlayerData> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PlayerDataMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PlayerData {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PlayerDataMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PlayerDataView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__EnemyData_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct EnemyData {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<EnemyData>
}

impl ::protobuf::Message for EnemyData {}

impl ::std::default::Default for EnemyData {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for EnemyData {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for EnemyData {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for EnemyData {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `EnemyData` is `Sync` because it does not implement interior mutability.
//    Neither does `EnemyDataMut`.
unsafe impl Sync for EnemyData {}

// SAFETY:
// - `EnemyData` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for EnemyData {}

impl ::protobuf::Proxied for EnemyData {
  type View<'msg> = EnemyDataView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for EnemyData {}

impl ::protobuf::MutProxied for EnemyData {
  type Mut<'msg> = EnemyDataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EnemyDataView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EnemyData>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EnemyDataView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EnemyDataView<'msg> {
  type Message = EnemyData;
}

impl ::std::fmt::Debug for EnemyDataView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for EnemyDataView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for EnemyDataView<'_> {
  fn default() -> EnemyDataView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    EnemyDataView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> EnemyDataView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EnemyData>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> EnemyData {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // entity_data: optional message Proto.EntityData
  pub fn has_entity_data(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn entity_data_opt(self) -> ::protobuf::Optional<super::EntityDataView<'msg>> {
        ::protobuf::Optional::new(self.entity_data(), self.has_entity_data())
  }
  pub fn entity_data(self) -> super::EntityDataView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::EntityDataView::new(::protobuf::__internal::Private, inner)
  }

  // enemy_type_id: optional uint64
  pub fn enemy_type_id(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `EnemyDataView` is `Sync` because it does not support mutation.
unsafe impl Sync for EnemyDataView<'_> {}

// SAFETY:
// - `EnemyDataView` is `Send` because while its alive a `EnemyDataMut` cannot.
// - `EnemyDataView` does not use thread-local data.
unsafe impl Send for EnemyDataView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for EnemyDataView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for EnemyDataView<'msg> {}

impl<'msg> ::protobuf::AsView for EnemyDataView<'msg> {
  type Proxied = EnemyData;
  fn as_view(&self) -> ::protobuf::View<'msg, EnemyData> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EnemyDataView<'msg> {
  fn into_view<'shorter>(self) -> EnemyDataView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<EnemyData> for EnemyDataView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EnemyData {
    let mut dst = EnemyData::new();
    let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(&mut dst, ::protobuf::__internal::Private);
    let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut dst, ::protobuf::__internal::Private);
    let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(&self, ::protobuf::__internal::Private);

    unsafe { ::protobuf::__internal::runtime::upb_Message_DeepCopy(
      dst_raw,
      src_raw,
      <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      dst_arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<EnemyData> for EnemyDataMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EnemyData {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for EnemyData {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn into_message_value_fuse_if_required(
      raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      mut val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let parent_arena = ::std::mem::ManuallyDrop::new(
          unsafe { ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena) });

      parent_arena.fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
      ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn from_message_value<'msg>(msg: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
        EnemyDataView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> EnemyDataMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, EnemyData>::wrap_raw(msg, arena) };
        EnemyDataMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EnemyDataMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EnemyData>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EnemyDataMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EnemyDataMut<'msg> {
  type Message = EnemyData;
}

impl ::std::fmt::Debug for EnemyDataMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for EnemyDataMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> EnemyDataMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EnemyData>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, EnemyData> {
    self.inner
  }

  pub fn to_owned(&self) -> EnemyData {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // entity_data: optional message Proto.EntityData
  pub fn has_entity_data(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_entity_data(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn entity_data_opt(&self) -> ::protobuf::Optional<super::EntityDataView<'_>> {
        ::protobuf::Optional::new(self.entity_data(), self.has_entity_data())
  }
  pub fn entity_data(&self) -> super::EntityDataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::EntityDataView::new(::protobuf::__internal::Private, inner)
  }
  pub fn entity_data_mut(&mut self) -> super::EntityDataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.arena()
       ).unwrap()
     };
     super::EntityDataMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_entity_data(&mut self,
    val: impl ::protobuf::IntoProxied<super::EntityData>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        0, child_ptr
      );
    }
  }

  // enemy_type_id: optional uint64
  pub fn enemy_type_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enemy_type_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `EnemyDataMut` does not perform any shared mutation.
// - `EnemyDataMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for EnemyDataMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for EnemyDataMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for EnemyDataMut<'msg> {}

impl<'msg> ::protobuf::AsView for EnemyDataMut<'msg> {
  type Proxied = EnemyData;
  fn as_view(&self) -> ::protobuf::View<'_, EnemyData> {
    EnemyDataView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EnemyDataMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, EnemyData>
  where
      'msg: 'shorter {
    EnemyDataView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for EnemyDataMut<'msg> {
  type MutProxied = EnemyData;
  fn as_mut(&mut self) -> EnemyDataMut<'msg> {
    EnemyDataMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EnemyDataMut<'msg> {
  fn into_mut<'shorter>(self) -> EnemyDataMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl EnemyData {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, EnemyData> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> EnemyDataView {
    EnemyDataView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> EnemyDataMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    EnemyDataMut::new(::protobuf::__internal::Private, inner)
  }

  // entity_data: optional message Proto.EntityData
  pub fn has_entity_data(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_entity_data(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn entity_data_opt(&self) -> ::protobuf::Optional<super::EntityDataView<'_>> {
        ::protobuf::Optional::new(self.entity_data(), self.has_entity_data())
  }
  pub fn entity_data(&self) -> super::EntityDataView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    let raw = submsg.map(|ptr| ptr.raw()).unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    super::EntityDataView::new(::protobuf::__internal::Private, inner)
  }
  pub fn entity_data_mut(&mut self) -> super::EntityDataMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.arena()
       ).unwrap()
     };
     super::EntityDataMut::from_parent(
       ::protobuf::__internal::Private,
       self.as_message_mut_inner(::protobuf::__internal::Private),
       ptr.raw())
  }
  pub fn set_entity_data(&mut self,
    val: impl ::protobuf::IntoProxied<super::EntityData>) {

    // The message and arena are dropped after the setter. The
    // memory remains allocated as we fuse the arena with the
    // parent message's arena.
    let mut child = val.into_proxied(::protobuf::__internal::Private);
    self.inner
      .arena()
      .fuse(::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut child, ::protobuf::__internal::Private));

    let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(&mut child, ::protobuf::__internal::Private);
    unsafe {
      self.inner.ptr_mut().set_base_field_message_at_index(
        0, child_ptr
      );
    }
  }

  // enemy_type_id: optional uint64
  pub fn enemy_type_id(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enemy_type_id(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

}  // impl EnemyData

impl ::std::ops::Drop for EnemyData {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for EnemyData {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for EnemyData {
  type Proxied = Self;
  fn as_view(&self) -> EnemyDataView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for EnemyData {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EnemyDataMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EnemyData {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__EnemyData_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$3,P".as_ptr(),
              4,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
        <super::EntityData as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__EnemyData_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__EnemyData_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EnemyData {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EnemyDataView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <EnemyData as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EnemyDataMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <EnemyData as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EnemyData {
  type Msg = EnemyData;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnemyData> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnemyData {
  type Msg = EnemyData;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnemyData> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EnemyDataMut<'_> {
  type Msg = EnemyData;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnemyData> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnemyDataMut<'_> {
  type Msg = EnemyData;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnemyData> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EnemyDataView<'_> {
  type Msg = EnemyData;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EnemyData> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EnemyDataMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for EnemyData {}
impl<'a> ::protobuf::MessageMutInterop<'a> for EnemyDataMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for EnemyDataView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Proto__PayloadClientInitializerData_msg_init: ::protobuf::__internal::runtime::MiniTablePtr =
    ::protobuf::__internal::runtime::MiniTablePtr(::std::ptr::null_mut());
#[allow(non_camel_case_types)]
pub struct PayloadClientInitializerData {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<PayloadClientInitializerData>
}

impl ::protobuf::Message for PayloadClientInitializerData {}

impl ::std::default::Default for PayloadClientInitializerData {
  fn default() -> Self {
    Self::new()
  }
}

impl ::protobuf::Parse for PayloadClientInitializerData {
  fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse(serialized)
  }

  fn parse_dont_enforce_required(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    Self::parse_dont_enforce_required(serialized)
  }
}

impl ::std::fmt::Debug for PayloadClientInitializerData {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadClientInitializerData {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

// SAFETY:
// - `PayloadClientInitializerData` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadClientInitializerDataMut`.
unsafe impl Sync for PayloadClientInitializerData {}

// SAFETY:
// - `PayloadClientInitializerData` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PayloadClientInitializerData {}

impl ::protobuf::Proxied for PayloadClientInitializerData {
  type View<'msg> = PayloadClientInitializerDataView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for PayloadClientInitializerData {}

impl ::protobuf::MutProxied for PayloadClientInitializerData {
  type Mut<'msg> = PayloadClientInitializerDataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadClientInitializerDataView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadClientInitializerData>,
  _phantom: ::std::marker::PhantomData<&'msg ()>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadClientInitializerDataView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PayloadClientInitializerDataView<'msg> {
  type Message = PayloadClientInitializerData;
}

impl ::std::fmt::Debug for PayloadClientInitializerDataView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadClientInitializerDataView<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    // SAFETY: `MINI_TABLE` is the one associated with `self.raw_msg()`.
    let encoded = unsafe {
      ::protobuf::__internal::runtime::wire::encode(self.raw_msg(),
          <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table())
    };
    encoded.map_err(|_| ::protobuf::SerializeError)
  }
}

impl ::std::default::Default for PayloadClientInitializerDataView<'_> {
  fn default() -> PayloadClientInitializerDataView<'static> {
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(::protobuf::__internal::runtime::ScratchSpace::zeroed_block()) };
    PayloadClientInitializerDataView::new(::protobuf::__internal::Private, inner)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadClientInitializerDataView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, PayloadClientInitializerData>) -> Self {
    Self { inner, _phantom: ::std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  pub fn to_owned(&self) -> PayloadClientInitializerData {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // players: repeated message Proto.PlayerData
  pub fn players(self) -> ::protobuf::RepeatedView<'msg, super::PlayerData> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::PlayerData>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // enemies: repeated message Proto.EnemyData
  pub fn enemies(self) -> ::protobuf::RepeatedView<'msg, super::EnemyData> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::EnemyData>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `PayloadClientInitializerDataView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadClientInitializerDataView<'_> {}

// SAFETY:
// - `PayloadClientInitializerDataView` is `Send` because while its alive a `PayloadClientInitializerDataMut` cannot.
// - `PayloadClientInitializerDataView` does not use thread-local data.
unsafe impl Send for PayloadClientInitializerDataView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadClientInitializerDataView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PayloadClientInitializerDataView<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadClientInitializerDataView<'msg> {
  type Proxied = PayloadClientInitializerData;
  fn as_view(&self) -> ::protobuf::View<'msg, PayloadClientInitializerData> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadClientInitializerDataView<'msg> {
  fn into_view<'shorter>(self) -> PayloadClientInitializerDataView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadClientInitializerData> for PayloadClientInitializerDataView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadClientInitializerData {
    let mut dst = PayloadClientInitializerData::new();
    let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(&mut dst, ::protobuf::__internal::Private);
    let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(&mut dst, ::protobuf::__internal::Private);
    let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(&self, ::protobuf::__internal::Private);

    unsafe { ::protobuf::__internal::runtime::upb_Message_DeepCopy(
      dst_raw,
      src_raw,
      <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      dst_arena.raw(),
    ) };
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<PayloadClientInitializerData> for PayloadClientInitializerDataMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> PayloadClientInitializerData {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::UpbTypeConversions for PayloadClientInitializerData {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }

    fn to_message_value(
        val: ::protobuf::View<'_, Self>) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn into_message_value_fuse_if_required(
      raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
      mut val: Self) -> ::protobuf::__internal::runtime::upb_MessageValue {
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let parent_arena = ::std::mem::ManuallyDrop::new(
          unsafe { ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena) });

      parent_arena.fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
      ::protobuf::__internal::runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn from_message_value<'msg>(msg: ::protobuf::__internal::runtime::upb_MessageValue)
        -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
        PayloadClientInitializerDataView::new(::protobuf::__internal::Private, inner)
    }

    unsafe fn from_message_mut<'msg>(msg: ::protobuf::__internal::runtime::RawMessage, arena: &'msg ::protobuf::__internal::runtime::Arena)
        -> PayloadClientInitializerDataMut<'msg> {
        let inner = unsafe { ::protobuf::__internal::runtime::MessageMutInner::<'msg, PayloadClientInitializerData>::wrap_raw(msg, arena) };
        PayloadClientInitializerDataMut::new(::protobuf::__internal::Private, inner)
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadClientInitializerDataMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadClientInitializerData>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PayloadClientInitializerDataMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PayloadClientInitializerDataMut<'msg> {
  type Message = PayloadClientInitializerData;
}

impl ::std::fmt::Debug for PayloadClientInitializerDataMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    let string = unsafe {
      ::protobuf::__internal::runtime::debug_string(
        self.raw_msg(),
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
      )
    };
    write!(f, "{}", string)
  }
}

impl ::protobuf::Serialize for PayloadClientInitializerDataMut<'_> {
  fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
    ::protobuf::AsView::as_view(self).serialize()
  }
}

#[allow(dead_code)]
impl<'msg> PayloadClientInitializerDataMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent<ParentT: ::protobuf::Message>(
             _private: ::protobuf::__internal::Private,
             parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
             msg: ::protobuf::__internal::runtime::RawMessage)
    -> Self {
    Self {
      inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::protobuf::__internal::Private, inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadClientInitializerData>) -> Self {
    Self { inner }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, PayloadClientInitializerData> {
    self.inner
  }

  pub fn to_owned(&self) -> PayloadClientInitializerData {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  // players: repeated message Proto.PlayerData
  pub fn players(&self) -> ::protobuf::RepeatedView<'_, super::PlayerData> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::PlayerData>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn players_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::PlayerData> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.arena(),
        ),
      )
    }
  }
  pub fn set_players(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::PlayerData>>) {
    let minitable_field = unsafe {
      ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        0
      )
    };
    let val = src.into_proxied(::protobuf::__internal::Private);
    let inner = val.inner(::protobuf::__internal::Private);

    self.arena().fuse(inner.arena());
    unsafe {
        let value_ptr: *const *const std::ffi::c_void =
            &(inner.raw().as_ptr() as *const std::ffi::c_void);
        ::protobuf::__internal::runtime::upb_Message_SetBaseField(self.raw_msg(),
          minitable_field,
          value_ptr as *const std::ffi::c_void);
    }
  }

  // enemies: repeated message Proto.EnemyData
  pub fn enemies(&self) -> ::protobuf::RepeatedView<'_, super::EnemyData> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::EnemyData>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn enemies_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::EnemyData> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
        self.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.arena(),
        ),
      )
    }
  }
  pub fn set_enemies(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::EnemyData>>) {
    let minitable_field = unsafe {
      ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        1
      )
    };
    let val = src.into_proxied(::protobuf::__internal::Private);
    let inner = val.inner(::protobuf::__internal::Private);

    self.arena().fuse(inner.arena());
    unsafe {
        let value_ptr: *const *const std::ffi::c_void =
            &(inner.raw().as_ptr() as *const std::ffi::c_void);
        ::protobuf::__internal::runtime::upb_Message_SetBaseField(self.raw_msg(),
          minitable_field,
          value_ptr as *const std::ffi::c_void);
    }
  }

}

// SAFETY:
// - `PayloadClientInitializerDataMut` does not perform any shared mutation.
// - `PayloadClientInitializerDataMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadClientInitializerDataMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for PayloadClientInitializerDataMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PayloadClientInitializerDataMut<'msg> {}

impl<'msg> ::protobuf::AsView for PayloadClientInitializerDataMut<'msg> {
  type Proxied = PayloadClientInitializerData;
  fn as_view(&self) -> ::protobuf::View<'_, PayloadClientInitializerData> {
    PayloadClientInitializerDataView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PayloadClientInitializerDataMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, PayloadClientInitializerData>
  where
      'msg: 'shorter {
    PayloadClientInitializerDataView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone()),
      _phantom: ::std::marker::PhantomData
    }
  }
}

impl<'msg> ::protobuf::AsMut for PayloadClientInitializerDataMut<'msg> {
  type MutProxied = PayloadClientInitializerData;
  fn as_mut(&mut self) -> PayloadClientInitializerDataMut<'msg> {
    PayloadClientInitializerDataMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PayloadClientInitializerDataMut<'msg> {
  fn into_mut<'shorter>(self) -> PayloadClientInitializerDataMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl PayloadClientInitializerData {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }

  fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
    self.inner.raw()
  }

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, PayloadClientInitializerData> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }

  pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
  }

  pub fn parse_dont_enforce_required(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
    let mut msg = Self::new();
    ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data).map(|_| msg)
  }

  pub fn as_view(&self) -> PayloadClientInitializerDataView {
    PayloadClientInitializerDataView::new(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner))
  }

  pub fn as_mut(&mut self) -> PayloadClientInitializerDataMut {
    let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner);
    PayloadClientInitializerDataMut::new(::protobuf::__internal::Private, inner)
  }

  // players: repeated message Proto.PlayerData
  pub fn players(&self) -> ::protobuf::RepeatedView<'_, super::PlayerData> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::PlayerData>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn players_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::PlayerData> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.arena(),
        ),
      )
    }
  }
  pub fn set_players(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::PlayerData>>) {
    let minitable_field = unsafe {
      ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        0
      )
    };
    let val = src.into_proxied(::protobuf::__internal::Private);
    let inner = val.inner(::protobuf::__internal::Private);

    self.arena().fuse(inner.arena());
    unsafe {
        let value_ptr: *const *const std::ffi::c_void =
            &(inner.raw().as_ptr() as *const std::ffi::c_void);
        ::protobuf::__internal::runtime::upb_Message_SetBaseField(self.raw_msg(),
          minitable_field,
          value_ptr as *const std::ffi::c_void);
    }
  }

  // enemies: repeated message Proto.EnemyData
  pub fn enemies(&self) -> ::protobuf::RepeatedView<'_, super::EnemyData> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::EnemyData>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn enemies_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::EnemyData> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
        self.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.arena(),
        ),
      )
    }
  }
  pub fn set_enemies(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::EnemyData>>) {
    let minitable_field = unsafe {
      ::protobuf::__internal::runtime::upb_MiniTable_GetFieldByIndex(
        <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        1
      )
    };
    let val = src.into_proxied(::protobuf::__internal::Private);
    let inner = val.inner(::protobuf::__internal::Private);

    self.arena().fuse(inner.arena());
    unsafe {
        let value_ptr: *const *const std::ffi::c_void =
            &(inner.raw().as_ptr() as *const std::ffi::c_void);
        ::protobuf::__internal::runtime::upb_Message_SetBaseField(self.raw_msg(),
          minitable_field,
          value_ptr as *const std::ffi::c_void);
    }
  }

}  // impl PayloadClientInitializerData

impl ::std::ops::Drop for PayloadClientInitializerData {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for PayloadClientInitializerData {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for PayloadClientInitializerData {
  type Proxied = Self;
  fn as_view(&self) -> PayloadClientInitializerDataView {
    self.as_view()
  }
}

impl ::protobuf::AsMut for PayloadClientInitializerData {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PayloadClientInitializerDataMut {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadClientInitializerData {
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTablePtr> =
        ::std::sync::OnceLock::new();
    ONCE_LOCK.get_or_init(|| unsafe {
      super::Proto__PayloadClientInitializerData_msg_init.0 =
          ::protobuf::__internal::runtime::upb_MiniTable_Build(
              "$GG".as_ptr(),
              3,
              ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA.with(|a| a.raw()),
              ::std::ptr::null_mut());
      let submessages = [
        <super::PlayerData as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
        <super::EnemyData as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
      ];
      let subenums = [
      ];
      assert!(::protobuf::__internal::runtime::upb_MiniTable_Link(
          super::Proto__PayloadClientInitializerData_msg_init.0,
          submessages.as_ptr() as *const *const ::protobuf::__internal::runtime::upb_MiniTable,
          submessages.len(), subenums.as_ptr(), subenums.len()));
      ::protobuf::__internal::runtime::MiniTablePtr(super::Proto__PayloadClientInitializerData_msg_init.0)
    }).0
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadClientInitializerData {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadClientInitializerDataView<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadClientInitializerData as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PayloadClientInitializerDataMut<'_> {
  #[inline(always)]
  fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
    <PayloadClientInitializerData as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadClientInitializerData {
  type Msg = PayloadClientInitializerData;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadClientInitializerData> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadClientInitializerData {
  type Msg = PayloadClientInitializerData;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadClientInitializerData> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PayloadClientInitializerDataMut<'_> {
  type Msg = PayloadClientInitializerData;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadClientInitializerData> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadClientInitializerDataMut<'_> {
  type Msg = PayloadClientInitializerData;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadClientInitializerData> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PayloadClientInitializerDataView<'_> {
  type Msg = PayloadClientInitializerData;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<PayloadClientInitializerData> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PayloadClientInitializerDataMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// upb kernel doesn't support any owned message or message mut interop.
impl ::protobuf::OwnedMessageInterop for PayloadClientInitializerData {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PayloadClientInitializerDataMut<'a> {}

impl<'a> ::protobuf::MessageViewInterop<'a> for PayloadClientInitializerDataView<'a> {
  unsafe fn __unstable_wrap_raw_message(
    msg: &'a *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
    msg: *const ::std::ffi::c_void) -> Self {
    let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _).unwrap();
    let inner = unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw) };
    Self::new(::protobuf::__internal::Private, inner)
  }
  fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
    self.inner.raw().as_ptr() as *const _
  }
}

