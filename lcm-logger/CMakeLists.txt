add_executable(lcm-logger lcm_logger.c glib_util.c)
# Use pkg-config libraries for better static linking support
if(GLIB_FOUND)
  target_link_libraries(lcm-logger PRIVATE lcm-static ${lcm-winport} ${GLIB_LIBRARIES})
  target_include_directories(lcm-logger PRIVATE ${GLIB_INCLUDE_DIRS})
  target_compile_options(lcm-logger PRIVATE ${GLIB_CFLAGS_OTHER})
else()
  target_link_libraries(lcm-logger PRIVATE lcm-static ${lcm-winport} GLib2::glib)
endif()

# Add Windows libraries for static linking
if(WIN32)
  target_link_libraries(lcm-logger PRIVATE 
    wsock32 ws2_32 winmm kernel32 user32 gdi32 winspool shell32 
    ole32 oleaut32 uuid comdlg32 advapi32
  )
endif()

add_executable(lcm-logplayer lcm_logplayer.c)
# Use pkg-config libraries for better static linking support
if(GLIB_FOUND)
  target_link_libraries(lcm-logplayer PRIVATE lcm-static ${lcm-winport} ${GLIB_LIBRARIES})
  target_include_directories(lcm-logplayer PRIVATE ${GLIB_INCLUDE_DIRS})
  target_compile_options(lcm-logplayer PRIVATE ${GLIB_CFLAGS_OTHER})
else()
  target_link_libraries(lcm-logplayer PRIVATE lcm-static ${lcm-winport} GLib2::glib)
endif()

# Add Windows libraries for static linking
if(WIN32)
  target_link_libraries(lcm-logplayer PRIVATE 
    wsock32 ws2_32 winmm kernel32 user32 gdi32 winspool shell32 
    ole32 oleaut32 uuid comdlg32 advapi32
  )
endif()

install(TARGETS
  lcm-logger
  lcm-logplayer
  DESTINATION bin
)

install(FILES
  lcm-logger.1
  lcm-logplayer.1
  DESTINATION share/man/man1
)
