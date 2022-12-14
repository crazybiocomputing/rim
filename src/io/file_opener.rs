//
//  RIM - Rust Image
//  Copyright (C) 2022  Jean-Christophe Taveau.
//
//  This file is part of RIM
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with RIM.  If not, see <http://www.gnu.org/licenses/>.
 
 
 
//!
//! Opens or reverts an image specified by a FileInfo object. Images can
//! be loaded from either a file (directory+fileName) or a URL (url+fileName).
//! # Example
//! 
//! ```rust
//!   pub class FileInfo_Test implements PlugIn {
//!     pub fn run(String arg) {
//!       FileInfo fi = new FileInfo();
//!       fi.width = 256;
//!       fi.height = 254;
//!       fi.offset = 768;
//!       fi.fileName = "blobs.tif";
//!       fi.directory = "/Users/wayne/Desktop/";
//!       new FileOpener(fi).open();
//!     }  
//!   }    
//! ```
//!
struct FileOpener {

    pub fi: FileInfo;
    pub width: u32,
    pub height: u32;
    show_conflict_message: bool;
    min_value: f64;
    max_value: f64;
    silent_mode: bool;
}

impl FileOpener {
    pub new( &fi: FileInfo) -> Self {
        FileOpener {
            fi: &fi;
            width: fi.width,
            height: fi.height,
            show_conflict_message: true,
            min_value: f64, 
            max_value: f64,
            silent_mode: true,
        }
        if RIM.debug_mode {
          log!("FileInfo: ",fi);
        }
    }
}

    
    /// Opens the image and returns it has an ImagePlus object. 
    pub fn open() {
        ImagePlus imp=null;
        Object pixels;
        ProgressBar pb=null;
        ImageProcessor ip;
        
         cm : ColorModel = createColorModel(fi);
        if (fi.nImages>1)
            return openStack(cm, show);
        match fi.file_type {
            FileInfo::GRAY8 | 
            FileInfo::COLOR8 | 
            FileInfo::BITMAP | 
                pixels = readPixels(fi);
                if (pixels==null) return null;
                ip = new ByteProcessor(width, height, (byte[])pixels, cm);
                imp = new ImagePlus(fi.fileName, ip);
                break;
            FileInfo::GRAY16_SIGNED | 
            FileInfo::GRAY16_UNSIGNED | 
            FileInfo::GRAY12_UNSIGNED => 
                pixels = readPixels(fi);
                if (pixels==null) return null;
                ip = new ShortProcessor(width, height, (short[])pixels, cm);
                   imp = new ImagePlus(fi.fileName, ip);
                break;
            FileInfo::GRAY32_INT | 
            FileInfo::GRAY32_UNSIGNED | 
            FileInfo::GRAY32_FLOAT | 
            FileInfo::GRAY24_UNSIGNED | 
            FileInfo::GRAY64_FLOAT => 
                pixels = readPixels(fi);
                if (pixels==null) return null;
                ip = new FloatProcessor(width, height, (float[])pixels, cm);
                   imp = new ImagePlus(fi.fileName, ip);
                break;
            FileInfo::RGB => 
            FileInfo::BGR => 
            FileInfo::ARGB => 
            FileInfo::ABGR => 
            FileInfo::BARG => 
            FileInfo::RGB_PLANAR => 
            FileInfo::CMYK => 
                pixels = readPixels(fi);
                if (pixels==null) return null;
                ip = new ColorProcessor(width, height, (int[])pixels);
                if (fi.file_type==FileInfo::CMYK)
                    ip.invert();
                imp = new ImagePlus(fi.fileName, ip);
                break;
            FileInfo::RGB48 => 
            FileInfo::RGB48_PLANAR => 
                boolean planar = fi.file_type==FileInfo::RGB48_PLANAR;
                Object[] pixelArray = (Object[])readPixels(fi);
                if (pixelArray==null) return null;
                int nChannels = 3;
                ImageStack stack = new ImageStack(width, height);
                stack.addSlice("Red", pixelArray[0]);
                stack.addSlice("Green", pixelArray[1]);
                stack.addSlice("Blue", pixelArray[2]);
                if (fi.samplesPerPixel==4 && pixelArray.length==4) {
                    stack.addSlice("Gray", pixelArray[3]);
                    nChannels = 4;
                }
                imp = new ImagePlus(fi.fileName, stack);
                imp.setDimensions(nChannels, 1, 1);
                if (planar)
                    imp.getProcessor().resetMinAndMax();
                imp.setFileInfo(fi);
                int mode = RIM.COMPOSITE;
                if (fi.description!=null) {
                    if (fi.description.indexOf("mode=color")!=-1)
                    mode = RIM.COLOR;
                    else if (fi.description.indexOf("mode=gray")!=-1)
                    mode = RIM.GRAYSCALE;
                }
                imp = new CompositeImage(imp, mode);
                if (!planar && fi.displayRanges==null) {
                    if (nChannels==4)
                        ((CompositeImage)imp).resetDisplayRanges();
                    else {
                        for (int c=1; c<=3; c++) {
                            imp.setPosition(c, 1, 1);
                            imp.setDisplayRange(min_value, max_value);
                        }
                        imp.setPosition(1, 1, 1);
                       }
                }
                if (fi.whiteIsZero) // cmyk?
                    RIM.run(imp, "Invert", "");
                break;
        }
        imp.setFileInfo(fi);
        setCalibration(imp);
        if (fi.info!=null)
            imp.setProperty("Info", fi.info);
        if (fi.sliceLabels!=null&&fi.sliceLabels.length==1&&fi.sliceLabels[0]!=null)
            imp.setProp("Slice_Label", fi.sliceLabels[0]);
        if (fi.plot!=null) try {
            Plot plot = new Plot(imp, new ByteArrayInputStream(fi.plot));
            imp.setProperty(Plot.PROPERTY_KEY, plot);
        } catch (Exception e) { RIM.handleException(e); }
        if (fi.roi!=null)
            decodeAndSetRoi(imp, fi);
        if (fi.overlay!=null)
            setOverlay(imp, fi.overlay);
        if (fi.properties!=null)
            imp.setProperties(fi.properties);
        return imp;
    }
    
    pub fn openProcessor() -> ImageProcessor {
        Object pixels;
        ProgressBar pb=null;
        ImageProcessor ip = null;        
        ColorModel cm = createColorModel(fi);
        switch (fi.file_type) {
            FileInfo::GRAY8 => 
            FileInfo::COLOR8 => 
            FileInfo::BITMAP => 
                pixels = readPixels(fi);
                if (pixels==null) return null;
                ip = new ByteProcessor(width, height, (byte[])pixels, cm);
                break;
            FileInfo::GRAY16_SIGNED => 
            FileInfo::GRAY16_UNSIGNED => 
            FileInfo::GRAY12_UNSIGNED => 
                pixels = readPixels(fi);
                if (pixels==null) return null;
                ip = new ShortProcessor(width, height, (short[])pixels, cm);
                break;
            FileInfo::GRAY32_INT => 
            FileInfo::GRAY32_UNSIGNED => 
            FileInfo::GRAY32_FLOAT => 
            FileInfo::GRAY24_UNSIGNED => 
            FileInfo::GRAY64_FLOAT => 
                pixels = readPixels(fi);
                if (pixels==null) return null;
                ip = new FloatProcessor(width, height, (float[])pixels, cm);
                break;
            FileInfo::RGB => 
            FileInfo::BGR => 
            FileInfo::ARGB => 
            FileInfo::ABGR => 
            FileInfo::BARG => 
            FileInfo::RGB_PLANAR => 
            FileInfo::CMYK => 
                pixels = readPixels(fi);
                if (pixels==null) return null;
                ip = new ColorProcessor(width, height, (int[])pixels);
                if (fi.file_type==FileInfo::CMYK)
                    ip.invert();
                break;
        }
        return ip;
    }

    // ???
    fn set_overlay(ImagePlus imp, byte[][] rois) {
        Overlay overlay = new Overlay();
        Overlay proto = null;
        for (int i=0; i<rois.length; i++) {
            Roi roi = RoiDecoder.openFromByteArray(rois[i]);
            if (roi==null)
                continue;
            if (proto==null) {
                proto = roi.getPrototypeOverlay();
                overlay.drawLabels(proto.getDrawLabels());
                overlay.drawNames(proto.getDrawNames());
                overlay.drawBackgrounds(proto.getDrawBackgrounds());
                overlay.setLabelColor(proto.getLabelColor());
                overlay.setLabelFont(proto.getLabelFont(), proto.scalableLabels());
            }
            overlay.add(roi);
        }
        imp.setOverlay(overlay);
    }

    // Opens a stack of images.
    fn open_stack(ColorModel cm, boolean show) -> ImagePlus {
        ImageStack stack = new ImageStack(fi.width, fi.height, cm);
        long skip = fi.getOffset();
        Object pixels;
        try {
            ImageReader reader = new ImageReader(fi);
            InputStream is = createInputStream(fi);
            if (is==null)
                return null;
            RIM.resetEscape();
            for (int i=1; i<=fi.nImages; i++) {
                if (!silent_mode)
                    RIM.showStatus("Reading: " + i + "/" + fi.nImages);
                if (RIM.escapePressed()) {
                    RIM.beep();
                    RIM.showProgress(1.0);
                    silent_mode = false;
                    return null;
                }
                pixels = reader.readPixels(is, skip);
                if (pixels==null)
                    break;
                stack.addSlice(null, pixels);
                skip = fi.getGap();
                if (!silent_mode)
                    RIM.showProgress(i, fi.nImages);
            }
            is.close();
        }
        catch (Exception e) {
            RIM.log("" + e);
        }
        catch(OutOfMemoryError e) {
            RIM.outOfMemory(fi.fileName);
            stack.trim();
        }
        if (!silent_mode) RIM.showProgress(1.0);
        if (stack.size()==0)
            return null;
        if (fi.sliceLabels!=null && fi.sliceLabels.length<=stack.size()) {
            for (int i=0; i<fi.sliceLabels.length; i++)
                stack.setSliceLabel(fi.sliceLabels[i], i+1);
        }
        ImagePlus imp = new ImagePlus(fi.fileName, stack);
        if (fi.info!=null)
            imp.setProperty("Info", fi.info);
        if (fi.roi!=null)
            decodeAndSetRoi(imp, fi);
        if (fi.overlay!=null)
            setOverlay(imp, fi.overlay);
        if (fi.properties!=null)
            imp.setProperties(fi.properties);

        imp.setFileInfo(fi);
        setCalibration(imp);
        ImageProcessor ip = imp.getProcessor();
        if (ip.getMin()==ip.getMax())  // find stack min and max if first slice is blank
            setStackDisplayRange(imp);
        if (!silent_mode) RIM.showProgress(1.0);
        return imp;
    }
    
    private fn decodeAndSetRoi(ImagePlus imp, FileInfo fi) {
        Roi roi = RoiDecoder.openFromByteArray(fi.roi);
        imp.setRoi(roi);
        if ((roi instanceof PointRoi) && ((PointRoi)roi).getNCounters()>1) 
            RIM.setTool("multi-point");
    }

    fn setStackDisplayRange(ImagePlus imp) {
        ImageStack stack = imp.getStack();
        double min = Double.MAX_VALUE;
        double max = -Double.MAX_VALUE;
        int n = stack.size();
        for (int i=1; i<=n; i++) {
            if (!silent_mode)
                RIM.showStatus("Calculating stack min and max: "+i+"/"+n);
            ImageProcessor ip = stack.getProcessor(i);
            ip.resetMinAndMax();
            if (ip.getMin()<min)
                min = ip.getMin();
            if (ip.getMax()>max)
                max = ip.getMax();
        }
        imp.getProcessor().setMinAndMax(min, max);
        imp.updateAndDraw();
    }
    

    fn setCalibration(ImagePlus imp) {
        if (fi.file_type==FileInfo::GRAY16_SIGNED) {
            if (RIM.debugMode) RIM.log("16-bit signed");
             imp.getLocalCalibration().setSigned16BitCalibration();
        }
        Properties props = decodeDescriptionString(fi);
        Calibration cal = imp.getCalibration();
        boolean calibrated = false;
        if (fi.pixelWidth>0.0 && fi.unit!=null) {
            double threshold = 0.001;
            if (fi.description!=null && fi.description.startsWith("ImageJ"))
                threshold = 0.0001;
            if (Prefs.convertToMicrons && fi.pixelWidth<=threshold && fi.unit.equals("cm")) {
                fi.pixelWidth *= 10000.0;
                fi.pixelHeight *= 10000.0;
                if (fi.pixelDepth!=1.0)
                    fi.pixelDepth *= 10000.0;
                fi.unit = "um";
            }
            cal.pixelWidth = fi.pixelWidth;
            cal.pixelHeight = fi.pixelHeight;
            cal.pixelDepth = fi.pixelDepth;
            cal.setUnit(fi.unit);
            calibrated = true;
        }
        
        if (fi.valueUnit!=null) {
            if (imp.getBitDepth()==32)
                cal.set_valueUnit(fi.valueUnit);
            else {
                int f = fi.calibrationFunction;
                if ((f>=Calibration.STRAIGHT_LINE && f<=Calibration.EXP_RECOVERY && fi.coefficients!=null)
                || f==Calibration.UNCALIBRATED_OD) {
                    boolean zeroClip = props!=null && props.getProperty("zeroclip", "false").equals("true");    
                    cal.setFunction(f, fi.coefficients, fi.valueUnit, zeroClip);
                    calibrated = true;
                }
            }
        }
        
        if (calibrated)
            checkForCalibrationConflict(imp, cal);
        
        if (fi.frameInterval!=0.0)
            cal.frameInterval = fi.frameInterval;
        
        if (props==null)
            return;
                    
        cal.xOrigin = getDouble(props,"xorigin");
        cal.yOrigin = getDouble(props,"yorigin");
        cal.zOrigin = getDouble(props,"zorigin");
        cal.setInvertY(getBoolean(props, "inverty"));
        cal.info = props.getProperty("info");        
                
        cal.fps = getDouble(props,"fps");
        cal.loop = getBoolean(props, "loop");
        cal.frameInterval = getDouble(props,"finterval");
        cal.setTimeUnit(props.getProperty("tunit", "sec"));
        cal.setYUnit(props.getProperty("yunit"));
        cal.setZUnit(props.getProperty("zunit"));

        double displayMin = getDouble(props,"min");
        double displayMax = getDouble(props,"max");
        if (!(displayMin==0.0&&displayMax==0.0)) {
            int type = imp.getType();
            ImageProcessor ip = imp.getProcessor();
            if (type==ImagePlus.GRAY8 || type==ImagePlus.COLOR_256)
                ip.setMinAndMax(displayMin, displayMax);
            else if (type==ImagePlus.GRAY16 || type==ImagePlus.GRAY32) {
                if (ip.getMin()!=displayMin || ip.getMax()!=displayMax)
                    ip.setMinAndMax(displayMin, displayMax);
            }
        }
        
        if (getBoolean(props, "8bitcolor"))
            imp.setTypeToColor256(); // set type to COLOR_256
        
        int stackSize = imp.getStackSize();
        if (stackSize>1) {
            int channels = (int)getDouble(props,"channels");
            int slices = (int)getDouble(props,"slices");
            int frames = (int)getDouble(props,"frames");
            if (channels==0) channels = 1;
            if (slices==0) slices = 1;
            if (frames==0) frames = 1;
            //RIM.log("setCalibration: "+channels+"  "+slices+"  "+frames);
            if (channels*slices*frames==stackSize) {
                imp.setDimensions(channels, slices, frames);
                if (getBoolean(props, "hyperstack"))
                    imp.setOpenAsHyperStack(true);
            }
        }
    }

    // Use dialog window ???
    fn checkForCalibrationConflict(ImagePlus imp, Calibration cal) {
        Calibration gcal = imp.getGlobalCalibration();
        if  (gcal==null || !show_conflict_message || RIM.isMacro())
            return;
        if (cal.pixelWidth==gcal.pixelWidth && cal.getUnit().equals(gcal.getUnit()))
            return;
        // GenericDialog gd = new GenericDialog(imp.getTitle());
        // gd.addMessage("The calibration of this image conflicts\nwith the current global calibration.");
        // gd.addCheckbox("Disable_Global Calibration", true);
        // gd.addCheckbox("Disable_these Messages", false);
        // gd.showDialog();
        // if (gd.wasCanceled()) return;
        // boolean disable = gd.getNextBoolean();
        if (disable) {
            imp.setGlobalCalibration(null);
            imp.setCalibration(cal);
            WindowManager.repaintImageWindows();
        }
        // boolean dontShow = gd.getNextBoolean();
        // if (dontShow) show_conflict_message = false;
    }

    /** Returns an IndexColorModel for the image specified by this FileInfo:: */
    pub ColorModel createColorModel(FileInfo fi) {
        if (fi.lutSize>0)
            return new IndexColorModel(8, fi.lutSize, fi.reds, fi.greens, fi.blues);
        else
            return LookUpTable.createGrayscaleColorModel(fi.whiteIsZero);
    }

    /** Returns an InputStream for the image described by this FileInfo:: */
    pub InputStream createInputStream(FileInfo fi) throws IOException, MalformedURLException {
        InputStream is = null;
        boolean gzip = fi.fileName!=null && (fi.fileName.endsWith(".gz")||fi.fileName.endsWith(".GZ"));
        if (fi.inputStream!=null)
            is = fi.inputStream;
        else if (fi.url!=null && !fi.url.equals(""))
            is = new URL(fi.url+fi.fileName).openStream();
        else {
            if (fi.directory!=null && fi.directory.length()>0 && !(fi.directory.endsWith(Prefs.separator)||fi.directory.endsWith("/")))
                fi.directory += Prefs.separator;
            File f = new File(fi.getFilePath());
            if (gzip) fi.compression = FileInfo::COMPRESSION_UNKNOWN;
            if (f==null || !f.exists() || f.isDirectory() || !validateFileInfo(f, fi))
                is = null;
            else
                is = new FileInputStream(f);
        }
        if (is!=null) {
            if (fi.compression>=FileInfo::LZW)
                is = new RandomAccessStream(is);
            else if (gzip)
                is = new GZIPInputStream(is, 50000);
        }
        return is;
    }
    
    static boolean validateFileInfo(File f, FileInfo fi) {
        long offset = fi.getOffset();
        long length = 0;
        if (fi.width<=0 || fi.height<=0) {
           error("Width or height <= 0.", fi, offset, length);
           return false;
        }
        if (offset>=0 && offset<1000L)
             return true;
        if (offset<0L) {
           error("Offset is negative.", fi, offset, length);
           return false;
        }
        if (fi.file_type==FileInfo::BITMAP || fi.compression!=FileInfo::COMPRESSION_NONE)
            return true;
        length = f.length();
        long size = fi.width*fi.height*fi.getBytesPerPixel();
        size = fi.nImages>1?size:size/4;
        if (fi.height==1) size = 0; // allows plugins to read info of unknown length at end of file
        if (offset+size>length) {
           error("Offset + image size > file length.", fi, offset, length);
           return false;
        }
        return true;
    }

    static fn error(String msg, FileInfo fi, long offset, long length) {
        String msg2 = "FileInfo parameter error. \n"
            +msg + "\n \n"
            +"  Width: " + fi.width + "\n"
            +"  Height: " + fi.height + "\n"
            +"  Offset: " + offset + "\n"
            +"  Bytes/pixel: " + fi.getBytesPerPixel() + "\n"
            +(length>0?"  File length: " + length + "\n":"");
        if (silent_mode) {
            RIM.log("Error opening "+fi.getFilePath());
            RIM.log(msg2);
        } else
            RIM.error("FileOpener", msg2);
    }


    /** Reads the pixel data from an image described by a FileInfo object. */
    fn read_pixels( &fi: FileInfo) {
        Object pixels = null;
        try {
            InputStream is = createInputStream(fi);
            if (is==null)
                return null;
            ImageReader reader = new ImageReader(fi);
            pixels = reader.readPixels(is);
            min_value = reader.min;
            max_value = reader.max;
            is.close();
        }
        catch (Exception e) {
            if (!Macro.MACRO_CANCELED.equals(e.getMessage()))
                RIM.handleException(e);
        }
        return pixels;
    }

    // Option???
    pub fn decodeDescriptionString(FileInfo fi) -> Properties {
        if (fi.description==null || fi.description.length()<7)
            return null;
        if (RIM.debugMode)
            RIM.log("Image Description: " + new String(fi.description).replace('\n',' '));
        if (!fi.description.startsWith("ImageJ"))
            return null;
        Properties props = new Properties();
        InputStream is = new ByteArrayInputStream(fi.description.getBytes());
        try {props.load(is); is.close();}
        catch (IOException e) {return null;}
        String dsUnit = props.getProperty("unit","");
        if ("cm".equals(fi.unit) && "um".equals(dsUnit)) {
            fi.pixelWidth *= 10000;
            fi.pixelHeight *= 10000;
        }
        fi.unit = dsUnit;
        Double n = getNumber(props,"cf");
        if (n!=null) fi.calibrationFunction = n.int_value();
        double c[] = new double[5];
        int count = 0;
        for (int i=0; i<5; i++) {
            n = getNumber(props,"c"+i);
            if (n==null) break;
            c[i] = n.double_value();
            count++;
        }
        if (count>=2) {
            fi.coefficients = new double[count];
            for (int i=0; i<count; i++)
                fi.coefficients[i] = c[i];            
        }
        fi.valueUnit = props.getProperty("vunit");
        n = getNumber(props,"images");
        if (n!=null && n.double_value()>1.0)
        fi.nImages = (int)n.double_value();
        n = getNumber(props, "spacing");
        if (n!=null) {
            double spacing = n.double_value();
            if (spacing<0) spacing = -spacing;
            fi.pixelDepth = spacing;
        }
        String name = props.getProperty("name");
        if (name!=null)
            fi.fileName = name;
        return props;
    }

    private Double getNumber(Properties props, String key) {
        String s = props.getProperty(key);
        if (s!=null) {
            try {
                return Double.valueOf(s);
            } catch (NumberFormatException e) {}
        }    
        return null;
    }
    
    private double getDouble(Properties props, String key) {
        Double n = getNumber(props, key);
        return n!=null?n.double_value():0.0;
    }
    
    private boolean getBoolean(Properties props, String key) {
        String s = props.getProperty(key);
        return s!=null&&s.equals("true")?true:false;
    }
    
    pub static fn setShowConflictMessage(boolean b) {
        show_conflict_message = b;
    }
    
    static fn setSilentMode(boolean mode) {
        silent_mode = mode;
    }


}

